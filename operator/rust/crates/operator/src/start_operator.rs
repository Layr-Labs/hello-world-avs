#![allow(missing_docs)]
use alloy_primitives::{eip191_hash_message, Address, FixedBytes, U256};
use alloy_provider::Provider;
use alloy_rpc_types::{BlockNumberOrTag, Filter};
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::{sol, SolEvent};
use chrono::Utc;
use dotenv::dotenv;
use eigen_client_elcontracts::{
    reader::ELChainReader,
    writer::{ELChainWriter, Operator},
};
use eigen_logging::{get_logger, init_logger, log_level::LogLevel, logger::Logger, EigenLogger};
use eigen_utils::get_signer;
use eyre::Result;
use once_cell::sync::Lazy;
use rand::RngCore;
use HelloWorldServiceManager::Task;

use eigen_utils::binding::ECDSAStakeRegistry;
use std::{env, str::FromStr};
use ECDSAStakeRegistry::SignatureWithSaltAndExpiry;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    HelloWorldServiceManager,
    "json_abi/HelloWorldServiceManager.json"
);

static KEY: Lazy<String> =
    Lazy::new(|| env::var("HOLESKY_PRIVATE_KEY").expect("failed to retrieve private key"));

pub static RPC_URL: Lazy<String> =
    Lazy::new(|| env::var("HOLESKY_RPC_URL").expect("failed to get rpc url from env"));

pub static HELLO_WORLD_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("HOLESKY_CONTRACT_ADDRESS")
        .expect("failed to get hello world contract address from env")
});

static DELEGATION_MANAGER_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("HOLESKY_DELEGATION_MANAGER_ADDRESS")
        .expect("failed to get delegation manager contract address from env")
});

static STAKE_REGISTRY_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("HOLESKY_STAKE_REGISTRY_ADDRESS")
        .expect("failed to get stake registry contract address from env")
});

static AVS_DIRECTORY_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("HOLESKY_AVS_DIRECTORY_ADDRESS")
        .expect("failed to get delegation manager contract address from env")
});

async fn sign_and_response_to_task(
    task_index: u32,
    task_created_block: u32,
    name: String,
) -> Result<()> {
    let provider = get_signer(KEY.clone(), &RPC_URL);

    let message = format!("Hello, {}", name);
    let msg_hash = eip191_hash_message(message);
    let wallet = PrivateKeySigner::from_str(&KEY.clone()).expect("failed to generate wallet ");

    let signature = wallet.sign_hash_sync(&msg_hash)?;

    println!("Signing and responding to task : {:?}", task_index);
    let hello_world_contract_address = Address::from_str(&HELLO_WORLD_CONTRACT_ADDRESS)
        .expect("wrong hello world contract address");
    let hello_world_contract =
        HelloWorldServiceManager::new(hello_world_contract_address, &provider);

    hello_world_contract
        .respondToTask(
            Task {
                name,
                taskCreatedBlock: task_created_block,
            },
            task_index,
            signature.as_bytes().into(),
        )
        .send()
        .await?
        .get_receipt()
        .await?;
    println!("Responded to task");
    Ok(())
}

/// Monitor new tasks
async fn monitor_new_tasks() -> Result<()> {
    let provider = get_signer(KEY.clone(), &RPC_URL);

    let hello_world_contract_address = Address::from_str(&HELLO_WORLD_CONTRACT_ADDRESS)
        .expect("wrong hello world contract address");

    let hello_world_contract =
        HelloWorldServiceManager::new(hello_world_contract_address, &provider);
    let word: &str = "EigenWorld";

    // If you want to send this tx to holesky , please uncomment the gas price and gas limit
    let _new_task_tx = hello_world_contract
        .createNewTask(word.to_owned())
        // .gas_price(20000000000)
        // .gas(300000)
        .send()
        .await?
        .get_receipt()
        .await?;

    let mut latest_processed_block = provider.get_block_number().await?;

    loop {
        println!("Monitoring for new tasks...");

        let filter = Filter::new()
            .address(hello_world_contract_address)
            .from_block(BlockNumberOrTag::Number(latest_processed_block));

        let logs = provider.get_logs(&filter).await?;

        for log in logs {
            match log.topic0() {
                Some(&HelloWorldServiceManager::NewTaskCreated::SIGNATURE_HASH) => {
                    let HelloWorldServiceManager::NewTaskCreated { taskIndex, task } = log
                        .log_decode()
                        .expect("Failed to decode log new task created")
                        .inner
                        .data;
                    println!("New task detected :Hello{:?} ", task.name);

                    let _ = sign_and_response_to_task(taskIndex, task.taskCreatedBlock, task.name)
                        .await;
                }
                _ => {}
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        let current_block = provider.get_block_number().await?;
        latest_processed_block = current_block;
    }
}

async fn register_operator(logger: EigenLogger) -> Result<()> {
    let wallet = PrivateKeySigner::from_str(&KEY).expect("failed to generate wallet ");

    let provider = get_signer(KEY.clone(), &RPC_URL);

    let hello_world_contract_address = Address::from_str(&HELLO_WORLD_CONTRACT_ADDRESS)
        .expect("wrong hello world contract address");
    let delegation_manager_contract_address: Address;
    let avs_directory_contract_address: Address;
    delegation_manager_contract_address = Address::from_str(&DELEGATION_MANAGER_CONTRACT_ADDRESS)
        .expect("wrong stake registry contract address");
    println!(
        "delegation manager :{}",
        delegation_manager_contract_address
    );
    avs_directory_contract_address = Address::from_str(&AVS_DIRECTORY_CONTRACT_ADDRESS)
        .expect("wrong stake registry contract address");
    let stake_registry_contract_address = Address::from_str(&STAKE_REGISTRY_CONTRACT_ADDRESS)
        .expect("wrong stake registry contract address");

    let default_slasher = Address::ZERO; // We don't need slasher for our example.
    let default_strategy = Address::ZERO; // We don't need strategy for our example.

    let elcontracts_reader_instance = ELChainReader::new(
        logger.clone(),
        default_slasher,
        delegation_manager_contract_address,
        avs_directory_contract_address,
        RPC_URL.clone(),
    );
    let elcontracts_writer_instance = ELChainWriter::new(
        delegation_manager_contract_address,
        default_strategy,
        elcontracts_reader_instance.clone(),
        RPC_URL.clone(),
        KEY.clone(),
    );

    let operator = Operator::new(
        wallet.address(),
        wallet.address(),
        Address::ZERO,
        0u32,
        None,
    );

    // let is_registered = elcontracts_reader_instance
    //     .is_operator_registered(wallet.address())
    //     .await
    //     .unwrap();
    // logger
    //     .tracing_logger
    //     .as_ref()
    //     .unwrap()
    //     .info(&format!("is registered {}", is_registered), &[""]);
    #[allow(unused_doc_comments)]
    ///In case you are running holesky. Comment the below register_as_operator call after the first
    /// call . Since we can register only once per operator.
    let _tx_hash = elcontracts_writer_instance
        .register_as_operator(operator)
        .await;
    logger
        .tracing_logger
        .as_ref()
        .unwrap()
        .info("Operator registered on EL successfully", &[""]);
    let mut salt = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut salt);

    let salt = FixedBytes::from_slice(&salt);
    let now = Utc::now().timestamp();
    let expiry: U256 = U256::from(now + 3600);
    let digest_hash = elcontracts_reader_instance
        .calculate_operator_avs_registration_digest_hash(
            wallet.address(),
            hello_world_contract_address,
            salt,
            expiry,
        )
        .await
        .expect("not able to calculate operator ");

    let signature = wallet.sign_hash_sync(&digest_hash)?;
    let operator_signature = SignatureWithSaltAndExpiry {
        signature: signature.as_bytes().into(),
        salt,
        expiry: expiry,
    };

    let contract_ecdsa_stake_registry =
        ECDSAStakeRegistry::new(stake_registry_contract_address, provider);
    let registeroperator_details_call = contract_ecdsa_stake_registry
        .registerOperatorWithSignature(wallet.clone().address(), operator_signature)
        .gas(200000);
    registeroperator_details_call
        .send()
        .await?
        .get_receipt()
        .await?;

    logger.tracing_logger.unwrap().info(
        &format!(
            "Operator registered on AVS successfully :{}",
            wallet.address()
        ),
        &[""],
    );

    Ok(())
}

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    init_logger(LogLevel::Info);
    let logger = get_logger();
    if let Err(e) = register_operator(logger).await {
        eprintln!("Failed to register operator: {:?}", e);
        return;
    }

    // Start the task monitoring as a separate async task to keep the process running
    tokio::spawn(async {
        if let Err(e) = monitor_new_tasks().await {
            eprintln!("Failed to monitor new tasks: {:?}", e);
        }
    });

    // Keep the process running indefinitely
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
