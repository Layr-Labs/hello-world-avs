#![allow(missing_docs)]
use alloy_network::{Ethereum, EthereumSigner};
use alloy_provider::RootProvider;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::{BlockNumberOrTag, Filter};
use alloy_sol_types::{sol, SolEvent};
use alloy_transport_http::Client;
use chrono::Utc;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use rand::RngCore;
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

use alloy_provider::fillers::{
    ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, SignerFiller,
};
use std::{env, str::FromStr};
use ECDSAStakeRegistry::SignatureWithSaltAndExpiry;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    HelloWorldServiceManager,
    "json_abi/HelloWorldServiceManager.json"
);

use eigen_utils::binding::ECDSAStakeRegistry;

static KEY: Lazy<String> =
    Lazy::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

pub static RPC_URL: Lazy<String> =
    Lazy::new(|| env::var("RPC_URL").expect("failed to get rpc url from env"));

pub static HELLO_WORLD_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("CONTRACT_ADDRESS").expect("failed to get hello world contract address from env")
});

static DELEGATION_MANAGER_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("DELEGATION_MANAGER_ADDRESS")
        .expect("failed to get delegation manager contract address from env")
});

static STAKE_REGISTRY_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("STAKE_REGISTRY_ADDRESS")
        .expect("failed to get stake registry contract address from env")
});

static AVS_DIRECTORY_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("AVS_DIRECTORY_ADDRESS")
        .expect("failed to get delegation manager contract address from env")
});
async fn sign_and_response_to_task(
    task_index: u32,
    task_created_block: u32,
    name: String,
) -> Result<()> {
    let provider = get_provider_with_wallet(KEY.clone());

    let message = format!("Hello, {}", name);
    let msg_hash = eip191_hash_message(message);
    let wallet = LocalWallet::from_str(&KEY.clone()).expect("failed to generate wallet ");

    let signature = wallet.sign_hash(&msg_hash).await?;

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
    let provider = get_provider_with_wallet(KEY.clone());

    let hello_world_contract_address = Address::from_str(&HELLO_WORLD_CONTRACT_ADDRESS)
        .expect("wrong hello world contract address");
    println!(
        "hello world contrat address:{:?}",
        hello_world_contract_address
    );
    let hello_world_contract =
        HelloWorldServiceManager::new(hello_world_contract_address, &provider);
    println!("hello contract :{:?}", hello_world_contract);
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

async fn register_operator() -> Result<()> {
    let wallet = LocalWallet::from_str(&KEY).expect("failed to generate wallet ");

    let provider = get_provider_with_wallet(KEY.clone());
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
    #[allow(unused_doc_comments)]
    ///In case you are running holesky. Comment the below register_as_operator call after the first
    /// call . Since we can register only once per operator.
    let _tx_hash = elcontracts_writer_instance
        .register_as_operator(operator)
        .await;
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

    let operator_signature = SignatureWithSaltAndExpiry {
        signature: signature.as_bytes().into(),
        salt,
        expiry: expiry,
    };

    let contract_ecdsa_stake_registry =
        ECDSAStakeRegistry::new(stake_registry_contract_address, provider);
    println!("initialize new ecdsa ");

    // If you wish to run on holesky, please deploy the stake registry contract(it's not deployed right now)
    // and uncomment the gas and gas_price
    let registeroperator_details = contract_ecdsa_stake_registry
        .registerOperatorWithSignature(wallet.clone().address(), operator_signature);
    let _tx = registeroperator_details
        // .gas(300000)
        // .gas_price(20000000000)
        .send()
        .await?
        .get_receipt()
        .await?;
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

pub fn get_provider_with_wallet(
    key: String,
) -> FillProvider<
    JoinFill<
        JoinFill<
            JoinFill<JoinFill<alloy_provider::Identity, GasFiller>, NonceFiller>,
            ChainIdFiller,
        >,
        SignerFiller<EthereumSigner>,
    >,
    RootProvider<alloy_transport_http::Http<Client>>,
    alloy_transport_http::Http<Client>,
    Ethereum,
> {
    let wallet = LocalWallet::from_str(&key.to_string()).expect("failed to generate wallet ");
    let url = Url::parse(&RPC_URL.clone()).expect("Wrong rpc url");
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(wallet.clone()))
        .on_http(url);

    return provider;
}
