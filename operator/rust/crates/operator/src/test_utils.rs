#![allow(missing_docs)]
use alloy_network::{Ethereum, EthereumSigner};
use alloy_provider::fillers::{
    ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, SignerFiller,
};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_rpc_types::{BlockNumberOrTag, Filter};
use alloy_sol_types::{sol, SolEvent};
use alloy_transport_http::Client;
use chrono::Utc;
use once_cell::sync::Lazy;
use rand::Rng;
use rand::RngCore;
use reqwest::Url;
use tokio::time::{self, Duration};
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
pub async fn monitor_new_tasks() -> Result<()> {
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

pub async fn register_operator() -> Result<()> {
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

    let provider = get_provider_with_wallet(KEY.clone());
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

    loop {
        interval.tick().await;
        let random_name = generate_random_name();
        let _ = create_new_task(&random_name).await;
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

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use eigen_utils::binding::DelegationManager::{self, isOperatorReturn};
    use HelloWorldServiceManager::latestTaskNumReturn;
    #[tokio::test]
    async fn test_register_operator() {
        dotenv().ok();
        let _ = register_operator().await;

        let wallet = LocalWallet::from_str(&KEY).expect("failed to generate wallet ");
        let url = Url::parse(&RPC_URL).expect("Wrong rpc url");
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .signer(EthereumSigner::from(wallet.clone()))
            .on_http(url);

        let delegation_manager_contract_address =
            Address::from_str(&DELEGATION_MANAGER_CONTRACT_ADDRESS)
                .expect("wrong delegation manager contract address");

        let contract_delegation_manager =
            DelegationManager::new(delegation_manager_contract_address, &provider);

        let is_operator = contract_delegation_manager
            .isOperator(wallet.address())
            .call()
            .await
            .unwrap();

        let isOperatorReturn {
            _0: isoperator_bool,
        } = is_operator;

        assert!(isoperator_bool);
    }

    #[tokio::test]
    async fn test_spam_tasks() {
        dotenv().ok();
        let provider = get_provider_with_wallet(KEY.clone());
        let hello_world_contract_address = Address::from_str(&HELLO_WORLD_CONTRACT_ADDRESS)
            .expect("wrong hello world contract address");
        let hello_world_contract =
            HelloWorldServiceManager::new(hello_world_contract_address, &provider);

        let latest_task_num = hello_world_contract.latestTaskNum().call().await.unwrap();

        let latestTaskNumReturn { _0: task_num } = latest_task_num;
        let _ = create_new_task("HelloEigen").await;

        let latest_task_num_after_creating_task =
            hello_world_contract.latestTaskNum().call().await.unwrap();
        let latestTaskNumReturn {
            _0: task_num_after_task,
        } = latest_task_num_after_creating_task;

        assert_eq!(task_num + 1, task_num_after_task);
    }
}
