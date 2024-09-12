#![allow(missing_docs)]
use alloy_primitives::Address;
use alloy_sol_types::sol;
use dotenv::dotenv;
use eigen_logging::{get_logger, init_logger, log_level::LogLevel, logger::Logger};
use eigen_utils::get_signer;
use eyre::Result;
use once_cell::sync::Lazy;
use rand::Rng;
use std::{env, str::FromStr};
use tokio::time::{self, Duration};

pub static RPC_URL: Lazy<String> =
    Lazy::new(|| env::var("RPC_URL").expect("failed to get rpc url from env"));

pub static HELLO_WORLD_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("CONTRACT_ADDRESS").expect("failed to get hello world contract address from env")
});
#[allow(unused)]
static KEY: Lazy<String> =
    Lazy::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    HelloWorldServiceManager,
    "json_abi/HelloWorldServiceManager.json"
);

#[allow(unused)]
/// Generate random task names from the given adjectives and nouns
fn generate_random_name() -> String {
    let adjectives = ["Quick", "Lazy", "Sleepy", "Noisy", "Hungry"];
    let nouns = ["Fox", "Dog", "Cat", "Mouse", "Bear"];

    let mut rng = rand::thread_rng();

    let adjective = adjectives[rng.gen_range(0..adjectives.len())];
    let noun = nouns[rng.gen_range(0..nouns.len())];
    let number: u16 = rng.gen_range(0..1000);

    format!("{}{}{}", adjective, noun, number)
}

#[allow(unused)]
/// Calls CreateNewTask function of the Hello world service manager contract
async fn create_new_task(task_name: &str) -> Result<()> {
    let hello_world_contract_address = Address::from_str(&HELLO_WORLD_CONTRACT_ADDRESS)
        .expect("wrong hello world contract address");

    let provider = get_signer(KEY.clone(), &RPC_URL);
    let hello_world_contract =
        HelloWorldServiceManager::new(hello_world_contract_address, provider);

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

#[allow(unused)]
/// Start creating tasks at every 15 seconds
async fn start_creating_tasks() {
    let mut interval = time::interval(Duration::from_secs(15));
    init_logger(LogLevel::Info);
    loop {
        interval.tick().await;
        let random_name = generate_random_name();
        get_logger().tracing_logger.unwrap().info(
            &format!("Creating new task with name: {} ", random_name),
            &["start_creating_tasks"],
        );
        let _ = create_new_task(&random_name).await;
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    dotenv().ok();
    start_creating_tasks().await;
}
