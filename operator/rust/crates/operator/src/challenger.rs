#![allow(missing_docs)]
use std::{collections::HashMap, env};

use alloy::{
    eips::BlockNumberOrTag, primitives::Address, providers::Provider, rpc::types::Filter,
    sol_types::SolEvent,
};
use dotenv::dotenv;
use eigensdk::{
    common::{get_provider, get_ws_provider},
    logging::{get_logger, init_logger, log_level::LogLevel},
    testing_utils::anvil_constants::ANVIL_WS_URL,
};
use eyre::Result;
use futures::StreamExt;
use hello_world_utils::{
    get_hello_world_service_manager,
    helloworldservicemanager::{HelloWorldServiceManager, IHelloWorldServiceManager::Task},
};
use once_cell::sync::Lazy;

pub const ANVIL_RPC_URL: &str = "http://localhost:8545";

static KEY: Lazy<String> =
    Lazy::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

/// Challenger struct
pub struct Challenger {
    service_manager_address: Address,
    rpc_url: String,
    ws_url: String,
    private_key: String,
    tasks: HashMap<u32, Task>,
    max_response_interval_blocks: u32,
}

/// Challenger implementation
impl Challenger {
    /// Create a new challenger
    pub async fn new(rpc_url: String, ws_url: String, private_key: String) -> Result<Self> {
        let service_manager_address = get_hello_world_service_manager().unwrap();
        let service_manager_contract =
            HelloWorldServiceManager::new(service_manager_address, get_provider(&rpc_url));
        let max_response_interval_blocks = service_manager_contract
            .MAX_RESPONSE_INTERVAL_BLOCKS()
            .call()
            .await?
            ._0;

        Ok(Self {
            service_manager_address,
            rpc_url,
            ws_url,
            private_key,
            tasks: HashMap::new(),
            max_response_interval_blocks,
        })
    }

    /// Start the challenger
    pub async fn start_challenger(&mut self) -> Result<()> {
        get_logger().info("challenger crate launched", "");

        let wa = get_ws_provider(&self.ws_url).await?;

        // Subscribe to NewTaskCreated event
        let new_task_created_filter = Filter::new()
            .address(self.service_manager_address)
            .event_signature(HelloWorldServiceManager::NewTaskCreated::SIGNATURE_HASH)
            .from_block(BlockNumberOrTag::Latest);
        let new_task_created_sub = wa.subscribe_logs(&new_task_created_filter).await?;

        let mut new_task_created_stream = new_task_created_sub.into_stream();

        // Subscribe to TaskResponded event
        let task_responded_filter = Filter::new()
            .address(self.service_manager_address)
            .event_signature(HelloWorldServiceManager::TaskResponded::SIGNATURE_HASH)
            .from_block(BlockNumberOrTag::Latest);
        let task_responded_sub = wa.subscribe_logs(&task_responded_filter).await?;

        let mut task_responded_stream = task_responded_sub.into_stream();

        loop {
            tokio::select! {
                Some(log) = task_responded_stream.next() => {
                    if let Ok(decoded) = log.log_decode::<HelloWorldServiceManager::TaskResponded>().inspect_err(|_| get_logger().info("Error decoding TaskResponded event", "")) {
                        let event = decoded.data();
                        get_logger().info(
                            &format!("TaskResponded: taskIndex={}, name={}, createdBlock={}, operator={}",
                            event.taskIndex,
                            event.task.name,
                            event.task.taskCreatedBlock,
                            event.operator
                        ),
                            "",
                        );
                    }
                },
                Some(log) = new_task_created_stream.next() => {
                    if let Ok(decoded) = log.log_decode::<HelloWorldServiceManager::NewTaskCreated>().inspect_err(|_| get_logger().info("Error decoding NewTaskCreated event", "")) {
                        let task_event = decoded.data();

                        get_logger().info(
                            &format!("NewTaskCreated: taskIndex={}, name={}, createdBlock={}",
                            task_event.taskIndex.clone(),
                            task_event.task.name.clone(),
                            task_event.task.taskCreatedBlock.clone()
                        ),
                            "",
                        );

                        let new_task = Task {
                            name: task_event.task.name.clone(),
                            taskCreatedBlock: task_event.task.taskCreatedBlock,
                        };

                        self.tasks.insert(task_event.taskIndex, new_task);

                    }
                },
                else => {
                    // If both streams are exhausted, break the loop.
                    get_logger().info("challenger:No more logs to process, exiting loop.", "");
                    break;
                }
            };
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    dotenv().ok();
    init_logger(LogLevel::Info);

    let mut challenger = Challenger::new(
        ANVIL_RPC_URL.to_string(),
        ANVIL_WS_URL.to_string(),
        KEY.clone(),
    )
    .await
    .unwrap();

    challenger.start_challenger().await.unwrap();
}
