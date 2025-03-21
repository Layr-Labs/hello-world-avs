#![allow(missing_docs)]
use std::{collections::HashMap, env, str::FromStr, sync::LazyLock, time::Duration};

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
use futures::{channel::oneshot, StreamExt};
use hello_world_utils::{
    get_hello_world_service_manager,
    helloworldservicemanager::{HelloWorldServiceManager, IHelloWorldServiceManager::Task},
};

static RPC_URL: LazyLock<String> =
    LazyLock::new(|| env::var("RPC_URL").expect("failed to retrieve RPC URL"));

static KEY: LazyLock<String> =
    LazyLock::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

const BLOCK_TIME: u64 = 12;

/// Challenger struct
#[derive(Debug)]
pub struct Challenger {
    service_manager_address: Address,
    rpc_url: String,
    ws_url: String,
    tasks: HashMap<u32, Task>,
    task_cancellers: HashMap<u32, oneshot::Sender<()>>,
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

        // Get the max response interval blocks that the operator has to respond to the task
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
            task_cancellers: HashMap::new(),
            max_response_interval_blocks,
            operator_address,
        })
    }

    /// Start the challenger to monitor new tasks and respond to them
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
                    let decode = log.log_decode::<HelloWorldServiceManager::NewTaskCreated>().ok();
                    if let Some(decoded) = decode {
                        self.handle_task_creation(decoded).await;
                    }
                },
                Some(log) = responded_stream.next() => {
                    let decode = log.log_decode::<HelloWorldServiceManager::TaskResponded>().ok();
                    if let Some(decoded) = decode {
                        self.handle_task_response(decoded).await;
                    }
                },
                else => {
                    // If both streams are exhausted, break the loop.
                    get_logger().info("challenger:No more logs to process, exiting loop.", "");
                    break;
                }
            }
        }

        Ok(())
    }

    /// Handle a new task creation and handle the time that an operator has to respond to the task
    async fn handle_task_creation(
        &mut self,
        decoded: Log<HelloWorldServiceManager::NewTaskCreated>,
    ) {
        let event = decoded.data().clone();
        let task_index = event.taskIndex;
        get_logger().info(
            &format!(
                "New task received: {} in block {}",
                task_index, event.task.taskCreatedBlock
            ),
            "",
        );

        // Save the task and create a cancellation channel
        self.tasks.insert(task_index, event.task.clone());
        let (cancel_tx, cancel_rx) = oneshot::channel();
        self.task_cancellers.insert(task_index, cancel_tx);

        // Calculate timeout based on max_response_interval_blocks
        let timeout_duration =
            Duration::from_secs((self.max_response_interval_blocks + 1) as u64 * BLOCK_TIME);

        let operator_address = self.operator_address;
        let rpc_url = self.rpc_url.clone();
        let service_manager_address = self.service_manager_address;

        tokio::spawn(async move {
            tokio::select! {
                _ = tokio::time::sleep(timeout_duration) => {
                    // If the timeout expires, slash the operator
                    get_logger().info(&format!("Timeout expired for task {}", task_index), "");
                    slash_operator(rpc_url, service_manager_address, operator_address, event.task, task_index).await.unwrap();
                },
                _ = cancel_rx => {
                    // If the task is responded in time, cancel the timeout timer
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
        get_logger().info(
            &format!(
                "Response received for task {} in block {}",
                task_index, event.task.taskCreatedBlock
            ),
            "",
        );

        if self.tasks.contains_key(&task_index) {
            if let Some(cancel_tx) = self.task_cancellers.remove(&task_index) {
                let _ = cancel_tx.send(());
            }
        }
    }
}

/// Execute the slashing of an operator
async fn slash_operator(
    rpc_url: String,
    service_manager_address: Address,
    operator_address: Address,
    task: Task,
    task_index: u32,
) -> Result<()> {
    let pr = get_signer(&KEY.to_string(), &rpc_url);
    let hello_world_contract = HelloWorldServiceManager::new(service_manager_address, &pr);

    get_logger().info(
        &format!(
            "Slashing operator {} in task {}",
            operator_address, task_index
        ),
        "",
    );

    let tx_result = hello_world_contract
        .slashOperator(task, task_index, operator_address)
        .send()
        .await?;

    get_logger().info(
        &format!("Slashing transaction sent: {}", tx_result.tx_hash()),
        "",
    );

    Ok(())
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_logger(LogLevel::Info);

    let mut challenger = Challenger::new(
        RPC_URL.to_string(),
        ANVIL_WS_URL.to_string(),
        KEY.to_string(),
    )
    .await
    .unwrap();

    challenger.start_challenger().await?;

    Ok(())
}
