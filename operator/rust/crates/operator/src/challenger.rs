#![allow(missing_docs)]
use std::{collections::HashMap, str::FromStr, time::Duration};

use alloy::{
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::Provider,
    rpc::types::{Filter, Log},
    signers::local::PrivateKeySigner,
    sol_types::SolEvent,
};
use dotenv::dotenv;
use eigensdk::{
    common::{get_provider, get_signer, get_ws_provider},
    logging::{get_logger, init_logger, log_level::LogLevel},
    testing_utils::anvil_constants::ANVIL_WS_URL,
};
use eyre::Result;
use futures::StreamExt;
use hello_world_utils::{
    get_hello_world_service_manager,
    helloworldservicemanager::{
        HelloWorldServiceManager::{self, TaskResponded},
        IHelloWorldServiceManager::Task,
    },
};
use tokio::sync::oneshot;

pub const ANVIL_RPC_URL: &str = "http://localhost:8545";

/// Challenger struct
#[derive(Debug)]
pub struct Challenger {
    service_manager_address: Address,
    rpc_url: String,
    ws_url: String,
    tasks: HashMap<u32, Task>,
    task_responses: HashMap<u32, TaskResponded>,
    max_response_interval_blocks: u32,
    task_cancellers: HashMap<u32, oneshot::Sender<()>>,
    operator_address: Address,
}

/// Challenger implementation
impl Challenger {
    /// Create a new challenger
    pub async fn new(rpc_url: String, ws_url: String, private_key: String) -> Result<Self> {
        let signer = PrivateKeySigner::from_str(&private_key)?;
        let operator_address = signer.address();

        let service_manager_address = get_hello_world_service_manager().unwrap();

        let pr = get_provider(&rpc_url);
        let service_manager_contract = HelloWorldServiceManager::new(service_manager_address, &pr);
        let max_response_interval_blocks = service_manager_contract
            .MAX_RESPONSE_INTERVAL_BLOCKS()
            .call()
            .await?
            ._0;

        Ok(Self {
            service_manager_address,
            rpc_url,
            ws_url,
            tasks: HashMap::new(),
            task_responses: HashMap::new(),
            task_cancellers: HashMap::new(),
            max_response_interval_blocks,
            operator_address,
        })
    }

    pub async fn start_challenger(&mut self) -> Result<()> {
        get_logger().info("Challenger started: monitoring tasks", "");

        let ws_provider = get_ws_provider(&self.ws_url).await?;

        // Subscribe to NewTaskCreated events
        let new_task_filter = Filter::new()
            .address(self.service_manager_address)
            .event_signature(HelloWorldServiceManager::NewTaskCreated::SIGNATURE_HASH)
            .from_block(BlockNumberOrTag::Latest);
        let mut new_task_stream = ws_provider
            .subscribe_logs(&new_task_filter)
            .await?
            .into_stream();

        // Subscribe to TaskResponded events
        let responded_filter = Filter::new()
            .address(self.service_manager_address)
            .event_signature(HelloWorldServiceManager::TaskResponded::SIGNATURE_HASH)
            .from_block(BlockNumberOrTag::Latest);
        let mut responded_stream = ws_provider
            .subscribe_logs(&responded_filter)
            .await?
            .into_stream();

        loop {
            tokio::select! {
                Some(log) = new_task_stream.next() => {
                    if let Ok(decoded) = log.log_decode::<HelloWorldServiceManager::NewTaskCreated>() {
                        self.handle_task_creation(decoded).await;
                    }
                },
                Some(log) = responded_stream.next() => {
                    if let Ok(decoded) = log.log_decode::<HelloWorldServiceManager::TaskResponded>() {
                        self.handle_task_response(decoded).await;
                    }
                },
                else => {
                    // If both streams are exhausted, break the loop.
                    get_logger().info("challenger:No more logs to process, exiting loop.", "");
                }
            }
        }
    }

    /// Handle a new task creation and handle the time that an operator has to respond to the task
    async fn handle_task_creation(
        &mut self,
        decoded: Log<HelloWorldServiceManager::NewTaskCreated>,
    ) {
        let event = decoded.data();
        let task_index = event.taskIndex;
        get_logger().info(&format!("New task received: {}", task_index), "");

        // Save the task and create a cancellation channel
        self.tasks.insert(task_index, event.task.clone());
        let (cancel_tx, cancel_rx) = oneshot::channel();
        self.task_cancellers.insert(task_index, cancel_tx);

        // Calculate timeout based on max_response_interval_blocks (assuming 12 seconds per block)
        let timeout_duration = Duration::from_secs(self.max_response_interval_blocks as u64 * 12);

        tokio::spawn(async move {
            tokio::select! {
                _ = tokio::time::sleep(timeout_duration) => {
                    get_logger().info(&format!("Timeout expired for task {}", task_index), "");
                    // TODO: Call the slash operator logic here
                    self.slash_operator(event.task, task_index, self.operator_address).await.unwrap();
                },
                _ = cancel_rx => {
                    get_logger().info(&format!("Task {} responded in time. Timer cancelled.", task_index), "");
                }
            }
        });
    }

    /// Handle a task response and cancel the timeout timer
    async fn handle_task_response(
        &mut self,
        decoded: Log<HelloWorldServiceManager::TaskResponded>,
    ) {
        let event = decoded.data();
        let task_index = event.taskIndex;
        get_logger().info(&format!("Response received for task {}", task_index), "");

        if self.tasks.contains_key(&task_index) {
            self.task_responses.insert(task_index, event.clone());
            if let Some(cancel_tx) = self.task_cancellers.remove(&task_index) {
                let _ = cancel_tx.send(());
            }
        }
    }

    /// Execute the slashing of an operator
    async fn slash_operator(&self, task: Task, task_index: u32, operator: Address) -> Result<()> {
        let pr = get_provider(&self.rpc_url);
        let hello_world_contract = HelloWorldServiceManager::new(self.service_manager_address, &pr);

        get_logger().info(
            &format!("Slashing operator {} in task {}", operator, task_index),
            "",
        );

        let tx_result = hello_world_contract
            .slashOperator(task, task_index, operator)
            .gas(500000)
            .send()
            .await?;

        get_logger().info(
            &format!("Slashing transaction sent: {}", tx_result.tx_hash()),
            "",
        );

        Ok(())
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    dotenv().ok();
    init_logger(LogLevel::Debug);

    let mut challenger = Challenger::new(ANVIL_RPC_URL.to_string(), ANVIL_WS_URL.to_string())
        .await
        .unwrap();

    if let Err(e) = challenger.start_challenger().await {
        get_logger().error(&format!("Error en el challenger: {}", e), "");
    }
}
