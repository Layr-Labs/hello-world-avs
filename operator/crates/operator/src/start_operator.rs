#![allow(missing_docs)]
use alloy_network::EthereumSigner;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::{BlockNumberOrTag, Filter};
use alloy_sol_types::{sol, SolEvent};
use chrono::Utc;
use once_cell::sync::Lazy;
use rand::RngCore;

use dotenv::dotenv;
use reqwest::Url;
use HelloWorldServiceManager::Task;

use alloy_primitives::{eip191_hash_message, Address, FixedBytes, U256};
use alloy_signer::Signer;
use alloy_signer_wallet::LocalWallet;
use eigen_client_elcontracts::{
    reader::ELChainReader,
    writer::{ELChainWriter, Operator},
};
use eyre::Result;

use std::{env, str::FromStr};
use ECDSAStakeRegistry::SignatureWithSaltAndExpiry;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    HelloWorldServiceManager,
    "json_abi/HelloWorldServiceManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ECDSAStakeRegistry,
    "json_abi/ECDSAStakeRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DelegationManager,
    "json_abi/DelegationManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IAVSDirectory,
    "json_abi/IAVSDirectory.json"
);

static KEY: Lazy<String> =
    Lazy::new(|| env::var("RUST_PRIVATE_KEY").expect("failed to retrieve private key"));

pub static RPC_URL: Lazy<String> =
    Lazy::new(|| env::var("RUST_RPC_URL").expect("failed to get rpc url from env"));

pub static HELLO_WORLD_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("RUST_CONTRACT_ADDRESS").expect("failed to get hello world contract address from env")
});

static DELEGATION_MANAGER_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("RUST_DELEGATION_MANAGER_ADDRESS")
        .expect("failed to get delegation manager contract address from env")
});

static STAKE_REGISTRY_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("RUST_STAKE_REGISTRY_ADDRESS")
        .expect("failed to get stake registry contract address from env")
});

static AVS_DIRECTORY_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("RUST_AVS_DIRECTORY_ADDRESS")
        .expect("failed to get delegation manager contract address from env")
});
async fn sign_and_response_to_task(
    task_index: u32,
    task_created_block: u32,
    name: String,
) -> Result<()> {
    let wallet = LocalWallet::from_str(&KEY).expect("failed to generate wallet ");

    let message = format!("Hello, {}", name);
    let msg_hash = eip191_hash_message(message);

    let signature = wallet.sign_hash(&msg_hash).await?;

    println!("Signing and responding to task : {:?}", task_index);
    let url = Url::parse(&RPC_URL.clone()).expect("Wrong rpc url");
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(wallet.clone()))
        .on_http(url);
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
    let wallet = LocalWallet::from_str(&KEY).expect("failed to generate wallet ");
    let url = Url::parse(&RPC_URL).expect("Wrong rpc url");
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(wallet.clone()))
        .on_http(url);

    let hello_world_contract_address = Address::from_str(&HELLO_WORLD_CONTRACT_ADDRESS)
        .expect("wrong hello world contract address");

    let hello_world_contract =
        HelloWorldServiceManager::new(hello_world_contract_address, &provider);
    let word: &str = "EigenWorld";
    let _new_task_tx = hello_world_contract
        .createNewTask(word.to_owned())
        .gas_price(20000000000)
        .gas(300000)
        .send()
        .await?
        .get_receipt()
        .await?;

    let mut latest_processed_block = provider.get_block_number().await?;

    loop {
        println!("Monitoring for new tasks...");
        let current_block = provider.get_block_number().await?;

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

        latest_processed_block = current_block;
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

async fn register_operator() -> Result<()> {
    let wallet = LocalWallet::from_str(&KEY).expect("failed to generate wallet ");

    let hello_world_contract_address = Address::from_str(&HELLO_WORLD_CONTRACT_ADDRESS)
        .expect("wrong hello world contract address");
    let delegation_manager_contract_address =
        Address::from_str(&DELEGATION_MANAGER_CONTRACT_ADDRESS)
            .expect("wrong delegation manager contract address");
    let stake_registry_contract_address = Address::from_str(&STAKE_REGISTRY_CONTRACT_ADDRESS)
        .expect("wrong stake registry contract address");
    let avs_directory_contract_address = Address::from_str(&AVS_DIRECTORY_CONTRACT_ADDRESS)
        .expect("wrong delegation manager contract address");

    let default_slasher = Address::ZERO; // We don't need slasher for our example.
    let default_strategy = Address::ZERO; // We don't need strategy for our example.
    let elcontracts_reader_instance = ELChainReader::new(
        default_slasher,
        delegation_manager_contract_address,
        default_strategy,
        avs_directory_contract_address,
        RPC_URL.clone(),
    );
    let _elcontracts_writer_instance = ELChainWriter::new(
        delegation_manager_contract_address,
        default_strategy,
        elcontracts_reader_instance.clone(),
        RPC_URL.clone(),
        wallet.clone(),
    );

    let _operator = Operator::new(
        wallet.address(),
        wallet.address(),
        Address::ZERO,
        0u32,
        None,
    );

    /// In our example , our operator is already registered. If you want to register
    /// an address as operator for the first time , uncomment it .
    // let tx_hash = elcontracts_writer_instance
    //     .register_as_operator(operator)
    //     .await;
    println!("Operator registered on EL successfully");
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

    let signature = wallet.sign_hash(&digest_hash).await?;
    let url = Url::parse(&RPC_URL).expect("Wrong rpc url");

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(wallet.clone()))
        .on_http(url);
    let _operator_signature = SignatureWithSaltAndExpiry {
        signature: signature.as_bytes().into(),
        salt,
        expiry: expiry,
    };

    let _contract_ecdsa_stake_registry =
        ECDSAStakeRegistry::new(stake_registry_contract_address, provider);
    println!("initialize new ecdsa ");
    // For Holesky , we currently don't have any deployed version of ecdsa stake registry contract.
    // If you wish to ude it , you can deploy it and uncomment below to register operator

    // let registeroperator_details = contract_ecdsa_stake_registry
    //     .registerOperatorWithSignature(wallet.clone().address(), operator_signature);
    // let tx  = registeroperator_details.gas(300000).gas_price(20000000000).send().await?.get_receipt().await?;
    println!(
        "Operator registered on AVS successfully :{:?}",
        wallet.address()
    );

    Ok(())
}

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    if let Err(e) = register_operator().await {
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
