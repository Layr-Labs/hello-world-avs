#![allow(missing_docs)]
use alloy::primitives::Address;
use dotenv::dotenv;
use eigensdk::common::get_signer;
use eigensdk::logging::{get_logger, init_logger, log_level::LogLevel};
use eyre::Result;
use hello_world_utils::get_anvil_hello_world_deployment_data;
use hello_world_utils::helloworldservicemanager::HelloWorldServiceManager;
use rand::Rng;
use std::env;
use std::sync::LazyLock;
use tokio::time::{self, Duration};

static RPC_URL: LazyLock<String> =
    LazyLock::new(|| env::var("RPC_URL").expect("failed to retrieve RPC URL"));

static KEY: LazyLock<String> =
    LazyLock::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

/// Generate random task names from the given adjectives and nouns
fn generate_random_name() -> String {
    let adjectives = ["Quick", "Lazy", "Sleepy", "Noisy", "Hungry"];
    let nouns = ["Fox", "Dog", "Cat", "Mouse", "Bear"];

    let mut rng = rand::rng();

    let adjective = adjectives[rng.random_range(0..adjectives.len())];
    let noun = nouns[rng.random_range(0..nouns.len())];
    let number: u16 = rng.random_range(0..1000);

    format!("{}{}{}", adjective, noun, number)
}

/// Calls CreateNewTask function of the Hello world service manager contract
pub async fn create_new_task(rpc_url: &str, task_name: &str) -> Result<()> {
    let hw_data = get_anvil_hello_world_deployment_data()?;
    let hello_world_contract_address: Address =
        hw_data.addresses.hello_world_service_manager.parse()?;
    let pr = get_signer(&KEY.clone(), rpc_url);
    let hello_world_contract = HelloWorldServiceManager::new(hello_world_contract_address, pr);

    let tx = hello_world_contract
        .createNewTask(task_name.to_string())
        .send()
        .await?
        .get_receipt()
        .await?;

    println!(
        "Transaction successfull with tx : {:?}",
        tx.transaction_hash
    );

    Ok(())
}

/// Start creating tasks at every 15 seconds
async fn start_creating_tasks() {
    let mut interval = time::interval(Duration::from_secs(6));
    init_logger(LogLevel::Info);
    loop {
        interval.tick().await;
        let random_name = generate_random_name();
        get_logger().info(
            &format!("Creating new task with name: {random_name}"),
            "start_creating_tasks",
        );
        let _ = create_new_task(&RPC_URL, &random_name).await;
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    dotenv().ok();
    start_creating_tasks().await;
}
