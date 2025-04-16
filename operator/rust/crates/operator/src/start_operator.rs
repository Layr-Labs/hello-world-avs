#![allow(missing_docs)]
use alloy::dyn_abi::DynSolValue;
use alloy::{
    primitives::{eip191_hash_message, keccak256, Address, U256},
    providers::Provider,
    rpc::types::{BlockNumberOrTag, Filter},
    signers::{local::PrivateKeySigner, SignerSync},
    sol_types::{SolEvent, SolValue},
};
use dotenv::dotenv;
use eigensdk::common::{get_provider, get_signer, get_ws_provider};
use eigensdk::logging::{get_logger, init_logger, log_level::LogLevel};
use eyre::Result;
use futures::StreamExt;
use hello_world_utils::helloworldservicemanager::{
    HelloWorldServiceManager, IHelloWorldServiceManager::Task,
};
use hello_world_utils::{get_hello_world_service_manager, register_operator};
use rand::Rng;
use std::sync::LazyLock;
use std::{env, str::FromStr};

static RPC_URL: LazyLock<String> =
    LazyLock::new(|| env::var("RPC_URL").expect("failed to retrieve RPC URL"));

static WS_URL: LazyLock<String> =
    LazyLock::new(|| env::var("WS_URL").expect("failed to retrieve WS URL"));

static KEY: LazyLock<String> =
    LazyLock::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

static OPERATOR_RESPONSE_PERCENTAGE: LazyLock<f64> = LazyLock::new(|| {
    env::var("OPERATOR_RESPONSE_PERCENTAGE")
        .expect("failed to retrieve operator response percentage")
        .parse::<f64>()
        .expect("failed to parse operator response percentage")
});

async fn sign_and_respond_to_task(
    rpc_url: &str,
    private_key: &str,
    task_index: u32,
    task_created_block: u32,
    name: String,
) -> Result<()> {
    let pr = get_signer(private_key, rpc_url);
    let signer = PrivateKeySigner::from_str(private_key)?;

    let message = format!("Hello, {}", name);
    let m_hash = eip191_hash_message(keccak256(message.abi_encode_packed()));
    let operators: Vec<DynSolValue> = vec![DynSolValue::Address(signer.address())];
    let signature: Vec<DynSolValue> =
        vec![DynSolValue::Bytes(signer.sign_hash_sync(&m_hash)?.into())];
    let current_block = U256::from(get_provider(rpc_url).get_block_number().await?);
    let signature_data = DynSolValue::Tuple(vec![
        DynSolValue::Array(operators.clone()),
        DynSolValue::Array(signature.clone()),
        DynSolValue::Uint(current_block, 32),
    ])
    .abi_encode_params();

    get_logger().info(
        &format!("Signing and responding to task: {task_index:?}"),
        "",
    );
    let hello_world_contract_address: Address = get_hello_world_service_manager()?;
    let hello_world_contract = HelloWorldServiceManager::new(hello_world_contract_address, &pr);

    let task = Task {
        name,
        taskCreatedBlock: task_created_block,
    };
    let response_hash = hello_world_contract
        .respondToTask(task, task_index, signature_data.into())
        .gas(500000)
        .send()
        .await?
        .get_receipt()
        .await?
        .transaction_hash;
    get_logger().info(
        &format!("Responded to task with tx hash {}", response_hash),
        "",
    );
    Ok(())
}

/// Monitor new tasks
async fn monitor_new_tasks(rpc_url: &str, private_key: &str) -> Result<()> {
    let hello_world_contract_address: Address = get_hello_world_service_manager()?;

    let ws_provider = get_ws_provider(&WS_URL).await?;

    // Subscribe to NewTaskCreated events
    let filter = Filter::new()
        .address(hello_world_contract_address)
        .event_signature(HelloWorldServiceManager::NewTaskCreated::SIGNATURE_HASH)
        .from_block(BlockNumberOrTag::Latest);
    let mut new_task_stream = ws_provider.subscribe_logs(&filter).await?.into_stream();

    // Process tasks when a new event is detected
    while let Some(log) = new_task_stream.next().await {
        if let Ok(decoded) = log.log_decode::<HelloWorldServiceManager::NewTaskCreated>() {
            let HelloWorldServiceManager::NewTaskCreated { taskIndex, task } = decoded.inner.data;
            get_logger().info(
                &format!(
                    "New task {} detected at block {}",
                    taskIndex, task.taskCreatedBlock
                ),
                "",
            );

            // There is a `OPERATOR_RESPONSE_PERCENTAGE` chance that the operator will respond to the task.
            // If the operator does not respond, the operator will be slashed.
            let should_respond = rand::rng().random_bool(*OPERATOR_RESPONSE_PERCENTAGE / 100.0);

            if should_respond {
                sign_and_respond_to_task(
                    rpc_url,
                    private_key,
                    taskIndex,
                    task.taskCreatedBlock,
                    task.name,
                )
                .await?;
            } else {
                get_logger().info(
                    &format!("Operator did not respond to task {}", taskIndex),
                    "",
                );
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
/// Monitor new tasks using polling
async fn monitor_new_tasks_polling(rpc_url: &str, private_key: &str) -> Result<()> {
    let pr = get_signer(private_key, rpc_url);
    let hello_world_contract_address: Address = get_hello_world_service_manager()?;
    let mut latest_processed_block = pr.get_block_number().await?;

    loop {
        let current_block = pr.get_block_number().await?;
        get_logger().info(
            &format!(
                "Monitoring for new tasks from block {} to {}",
                latest_processed_block, current_block
            ),
            "",
        );

        let filter = Filter::new()
            .address(hello_world_contract_address)
            .from_block(BlockNumberOrTag::Number(latest_processed_block))
            .to_block(BlockNumberOrTag::Number(current_block));

        let logs = pr.get_logs(&filter).await?;

        for log in logs {
            if let Some(&HelloWorldServiceManager::NewTaskCreated::SIGNATURE_HASH) = log.topic0() {
                let HelloWorldServiceManager::NewTaskCreated { taskIndex, task } = log
                    .log_decode()
                    .expect("Failed to decode log new task created")
                    .inner
                    .data;
                get_logger().info(
                    &format!("New task {} detected: Hello, {}", taskIndex, task.name),
                    "",
                );

                // There is a `OPERATOR_RESPONSE_PERCENTAGE` chance that the operator will respond to the task.
                // If the operator does not respond, the operator will be slashed.
                let should_respond = rand::rng().random_bool(*OPERATOR_RESPONSE_PERCENTAGE / 100.0);

                if should_respond {
                    let _ = sign_and_respond_to_task(
                        rpc_url,
                        private_key,
                        taskIndex,
                        task.taskCreatedBlock,
                        task.name,
                    )
                    .await;
                } else {
                    get_logger().info(
                        &format!("Operator did not respond to task {}", taskIndex),
                        "",
                    );
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
        latest_processed_block = current_block + 1;
    }
}

#[tokio::main]
pub async fn main() {
    use tokio::signal;
    dotenv().ok();
    init_logger(LogLevel::Info);
    let rpc_url = &RPC_URL;
    if let Err(e) = register_operator(rpc_url, &KEY).await {
        eprintln!("Failed to register operator: {:?}", e);
        return;
    }

    // Start the task monitoring as a separate async task to keep the process running
    tokio::spawn(async {
        if let Err(e) = monitor_new_tasks(rpc_url, &KEY).await {
            eprintln!("Failed to monitor new tasks: {:?}", e);
        }
    });

    // Wait for a Ctrl+C signal to gracefully shut down
    let _ = signal::ctrl_c().await;
    get_logger().info("Received Ctrl+C, shutting down...", "");
}
