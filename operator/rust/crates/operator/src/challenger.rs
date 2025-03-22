#![allow(missing_docs)]
use std::{collections::HashMap, env, str::FromStr, sync::LazyLock};

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
};
use eyre::{Ok, Result};
use futures::StreamExt;
use hello_world_utils::{
    get_hello_world_service_manager,
    helloworldservicemanager::{
        HelloWorldServiceManager::{self},
        IHelloWorldServiceManager::Task,
    },
};
use tokio::signal::{self};

static RPC_URL: LazyLock<String> =
    LazyLock::new(|| env::var("RPC_URL").expect("failed to retrieve RPC URL"));

static WS_URL: LazyLock<String> =
    LazyLock::new(|| env::var("WS_URL").expect("failed to retrieve WS URL"));

static KEY: LazyLock<String> =
    LazyLock::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

/// Challenger struct
#[derive(Debug)]
pub struct Challenger {
    service_manager_address: Address,
    rpc_url: String,
    ws_url: String,
    tasks: HashMap<u32, Task>,
    max_response_interval_blocks: u32,
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

        // Subscribe to new block events
        let mut block_stream = ws_provider.subscribe_blocks().await?.into_stream();

        loop {
            tokio::select! {
                Some(log) = new_task_stream.next() => {
                    let decode = log.log_decode::<HelloWorldServiceManager::NewTaskCreated>().ok();
                    if let Some(decoded) = decode {
                        self.handle_task_creation(decoded);
                    }
                },
                Some(log) = responded_stream.next() => {
                    let decode = log.log_decode::<HelloWorldServiceManager::TaskResponded>().ok();
                    if let Some(decoded) = decode {
                        self.handle_task_response(decoded);
                    }
                },
                Some(block) = block_stream.next() => {
                    self.check_tasks_timeout(block.number).await?;
                },
                _ = signal::ctrl_c() => {
                    get_logger().info("Received Ctrl+C, shutting down...", "");
                    break;
                }
            }
        }

        Ok(())
    }

    /// Handle a new task creation and handle the time that an operator has to respond to the task
    fn handle_task_creation(&mut self, decoded: Log<HelloWorldServiceManager::NewTaskCreated>) {
        let event = decoded.data().clone();
        let task_index = event.taskIndex;
        get_logger().info(
            &format!(
                "New task received: {} at block {}",
                task_index, event.task.taskCreatedBlock
            ),
            "",
        );

        // Save the task and create a cancellation channel
        self.tasks.insert(task_index, event.task.clone());
    }

    /// Handle a task response and cancel the timeout timer
    fn handle_task_response(&mut self, decoded: Log<HelloWorldServiceManager::TaskResponded>) {
        let event = decoded.data();
        let task_index = event.taskIndex;

        get_logger().info(&format!("Task {} responded", task_index), "");
        self.tasks.remove(&task_index);
    }

    /// Handle the expiration of a task and slash the operator if so.
    /// This function is called when a new block is received, iterates over all tasks and checks if they have expired
    /// If so, it slashes the operator and removes the task from the list.
    async fn check_tasks_timeout(&mut self, current_block: u64) -> Result<()> {
        for (task_index, task) in self.tasks.clone() {
            let task_created_block = task.taskCreatedBlock;
            let expiration_block = (task_created_block + self.max_response_interval_blocks) as u64;

            if current_block > expiration_block {
                get_logger().info(
                    &format!(
                        "Task {} expired at {}, operator didn't respond",
                        task_index, expiration_block
                    ),
                    "",
                );
                self.slash_operator(task.clone(), task_index).await?;
                self.tasks.remove(&task_index);
            }
        }

        Ok(())
    }

    /// Execute the slashing of an operator
    async fn slash_operator(&self, task: Task, task_index: u32) -> Result<()> {
        let pr = get_signer(&KEY.to_string(), &self.rpc_url);
        let hello_world_contract = HelloWorldServiceManager::new(self.service_manager_address, &pr);

        get_logger().info(
            &format!(
                "Slashing operator {} in task {}",
                self.operator_address, task_index
            ),
            "",
        );

        let tx_result = hello_world_contract
            .slashOperator(task, task_index, self.operator_address)
            .send()
            .await?;

        get_logger().info(
            &format!("Slashing transaction sent: {}", tx_result.tx_hash()),
            "",
        );

        Ok(())
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv().ok();
    init_logger(LogLevel::Info);

    let mut challenger = Challenger::new(RPC_URL.to_string(), WS_URL.to_string(), KEY.to_string())
        .await
        .unwrap();

    challenger.start_challenger().await?;

    Ok(())
}
