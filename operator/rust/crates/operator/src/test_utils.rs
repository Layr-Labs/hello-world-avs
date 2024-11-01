#![allow(missing_docs)]
use alloy::{
    dyn_abi::DynSolValue,
    network::EthereumWallet,
    primitives::{eip191_hash_message, keccak256, Address, FixedBytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{BlockNumberOrTag, Filter},
    signers::{local::PrivateKeySigner, SignerSync},
    sol_types::{SolEvent, SolValue},
};
use chrono::Utc;
use eigen_client_elcontracts::{
    reader::ELChainReader,
    writer::{ELChainWriter, Operator},
};
use eigen_logging::get_logger;
use eigen_utils::{get_provider, get_signer};
use eyre::Result;
use hello_world_utils::{
    ecdsastakeregistry::ISignatureUtils::SignatureWithSaltAndExpiry,
    helloworldservicemanager::{HelloWorldServiceManager, IHelloWorldServiceManager::Task},
    parse_avs_directory_address, parse_delegation_manager_address,
    parse_hello_world_service_manager, parse_stake_registry_address, EigenLayerData,
    HelloWorldData,
};
use once_cell::sync::Lazy;
use rand::Rng;
use rand::RngCore;
use reqwest::Url;
use std::{env, path::Path, str::FromStr};
use tokio::time::{self, Duration};

use hello_world_utils::ecdsastakeregistry::ECDSAStakeRegistry;

pub const ANVIL_RPC_URL: &str = "http://localhost:8545";

static KEY: Lazy<String> =
    Lazy::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

#[allow(unused)]
async fn sign_and_response_to_task(
    task_index: u32,
    task_created_block: u32,
    name: String,
) -> Result<()> {
    let pr = get_signer(&KEY.clone(), ANVIL_RPC_URL);
    let signer = PrivateKeySigner::from_str(&KEY.clone())?;

    let message = format!("Hello, {}", name);
    let m_hash = eip191_hash_message(keccak256(message.abi_encode_packed()));
    let operators: Vec<DynSolValue> = vec![DynSolValue::Address(signer.address())];
    let signature: Vec<DynSolValue> =
        vec![DynSolValue::Bytes(signer.sign_hash_sync(&m_hash)?.into())];
    let current_block = U256::from(get_provider(ANVIL_RPC_URL).get_block_number().await?);
    let signature_data = DynSolValue::Tuple(vec![
        DynSolValue::Array(operators.clone()),
        DynSolValue::Array(signature.clone()),
        DynSolValue::Uint(current_block, 32),
    ])
    .abi_encode_params();

    println!("Signing and responding to task : {:?}", task_index);

    let hello_world_contract_address: Address =
        parse_hello_world_service_manager("contracts/deployments/hello-world/31337.json")?;
    let hello_world_contract = HelloWorldServiceManager::new(hello_world_contract_address, &pr);

    let response_hash = hello_world_contract
        .respondToTask(
            Task {
                name,
                taskCreatedBlock: task_created_block,
            },
            task_index,
            signature_data.into(),
        )
        .gas(500000)
        .send()
        .await?
        .get_receipt()
        .await?
        .transaction_hash;
    println!("Responded to task with tx hash {}", response_hash);
    Ok(())
}

/// Monitor new tasks
#[allow(unused)]
async fn monitor_new_tasks() -> Result<()> {
    let pr = get_signer(&KEY.clone(), ANVIL_RPC_URL);
    let signer = PrivateKeySigner::from_str(&KEY.clone())?;
    let hello_world_contract_address: Address =
        parse_hello_world_service_manager("contracts/deployments/hello-world/31337.json")?;

    let mut latest_processed_block = pr.get_block_number().await?;

    loop {
        println!("Monitoring for new tasks...");

        let filter = Filter::new()
            .address(hello_world_contract_address)
            .from_block(BlockNumberOrTag::Number(latest_processed_block));

        let logs = pr.get_logs(&filter).await?;

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

        tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
        let current_block = pr.get_block_number().await?;
        latest_processed_block = current_block;
    }
}

async fn register_operator() -> Result<()> {
    let signer = PrivateKeySigner::from_str(&KEY.clone())?;
    let wallet = EthereumWallet::from(signer.clone());
    let pr = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(Url::from_str(&ANVIL_RPC_URL)?);

    let default_slasher = Address::ZERO; // We don't need slasher for our example.
    let default_strategy = Address::ZERO; // We don't need strategy for our example.

    let delegation_manager_address =
        parse_delegation_manager_address("contracts/deployments/core/31337.json")?;
    let avs_directory_address: Address =
        parse_avs_directory_address("contracts/deployments/core/31337.json")?;

    let elcontracts_reader_instance = ELChainReader::new(
        get_logger().clone(),
        default_slasher,
        delegation_manager_address,
        avs_directory_address,
        ANVIL_RPC_URL.to_string(),
    );
    let elcontracts_writer_instance = ELChainWriter::new(
        delegation_manager_address,
        default_strategy,
        Address::ZERO,
        elcontracts_reader_instance.clone(),
        ANVIL_RPC_URL.to_string(),
        KEY.clone(),
    );

    let operator = Operator {
        address: signer.address(),
        earnings_receiver_address: signer.address(),
        delegation_approver_address: Address::ZERO,
        staker_opt_out_window_blocks: 0u32,
        metadata_url: None,
    };

    let is_registered = elcontracts_reader_instance
        .is_operator_registered(signer.address())
        .await
        .unwrap();
    get_logger().info(&format!("is registered {}", is_registered), &"");
    #[allow(unused)]
    let tx_hash = elcontracts_writer_instance
        .register_as_operator(operator)
        .await?;
    get_logger().info(
        "Operator registered on EL successfully tx_hash {tx_hash:?}",
        &"",
    );
    let mut salt = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut salt);

    let salt = FixedBytes::from_slice(&salt);
    let now = Utc::now().timestamp();
    let expiry: U256 = U256::from(now + 3600);

    let hello_world_contract_address: Address =
        parse_hello_world_service_manager("contracts/deployments/hello-world/31337.json")?;
    let digest_hash = elcontracts_reader_instance
        .calculate_operator_avs_registration_digest_hash(
            signer.address(),
            hello_world_contract_address,
            salt,
            expiry,
        )
        .await?;

    let signature = signer.sign_hash_sync(&digest_hash)?;
    let operator_signature = SignatureWithSaltAndExpiry {
        signature: signature.as_bytes().into(),
        salt,
        expiry: expiry,
    };
    let stake_registry_address: Address =
        parse_stake_registry_address("contracts/deployments/hello-world/31337.json")?;
    let contract_ecdsa_stake_registry = ECDSAStakeRegistry::new(stake_registry_address, &pr);
    let registeroperator_details_call: alloy::contract::CallBuilder<
        _,
        &_,
        std::marker::PhantomData<ECDSAStakeRegistry::registerOperatorWithSignatureCall>,
        _,
    > = contract_ecdsa_stake_registry
        .registerOperatorWithSignature(operator_signature, signer.clone().address())
        .gas(500000);
    let register_hello_world_hash = registeroperator_details_call
        .send()
        .await?
        .get_receipt()
        .await?
        .transaction_hash;

    get_logger().info(
        &format!(
            "Operator registered on AVS successfully :{} , tx_hash :{}",
            signer.address(),
            register_hello_world_hash
        ),
        &"",
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
    let data = &format!("{}", env!("CARGO_MANIFEST_DIR"));
    let mut path = Path::new(data);
    for _ in 0..4 {
        path = path
            .parent()
            .expect("Reached the filesystem root, no more parent directories");
    }

    let s = &format!(
        "{}/contracts/deployments/hello-world/31337.json",
        &path.display()
    );
    let parsed: HelloWorldData =
        serde_json::from_str(&std::fs::read_to_string(s).unwrap()).unwrap();
    let hello_world_contract_address: Address =
        parsed.addresses.hello_world_service_manager.parse()?;
    let signer = PrivateKeySigner::from_str(&KEY.clone())?;
    let wallet = EthereumWallet::from(signer);
    let pr = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(Url::from_str(&ANVIL_RPC_URL)?);
    let hello_world_contract = HelloWorldServiceManager::new(hello_world_contract_address, pr);

    let tx = hello_world_contract
        .createNewTask(task_name.to_string())
        .send()
        .await?
        .get_receipt()
        .await?;

    get_logger().info(
        &format!(
            "Transaction successfull with tx : {:?}",
            tx.transaction_hash
        ),
        "",
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

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use eigen_logging::init_logger;
    use eigen_utils::delegationmanager::DelegationManager::{self, isOperatorReturn};
    use serial_test::serial;
    use std::path::Path;
    use HelloWorldServiceManager::latestTaskNumReturn;
    #[tokio::test]
    #[serial]
    async fn test_register_operator() {
        dotenv().ok();
        init_logger(eigen_logging::log_level::LogLevel::Info);
        let _ = register_operator().await;

        let signer = PrivateKeySigner::from_str(&KEY.clone()).unwrap();
        let wallet = EthereumWallet::from(signer.clone());
        let pr = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(Url::from_str(&ANVIL_RPC_URL).unwrap());
        let data = &format!("{}", env!("CARGO_MANIFEST_DIR"));
        let mut path = Path::new(data);
        for _ in 0..4 {
            path = path
                .parent()
                .expect("Reached the filesystem root, no more parent directories");
        }

        let s = &format!("{}/contracts/deployments/core/31337.json", &path.display());
        let el_parsed: EigenLayerData =
            serde_json::from_str(&std::fs::read_to_string(s).unwrap()).unwrap();
        let delegation_manager_address: Address = el_parsed.addresses.delegation.parse().unwrap();
        let contract_delegation_manager = DelegationManager::new(delegation_manager_address, &pr);

        let is_operator = contract_delegation_manager
            .isOperator(signer.address())
            .call()
            .await
            .unwrap();

        let isOperatorReturn {
            _0: isoperator_bool,
        } = is_operator;

        assert!(isoperator_bool);
    }

    #[tokio::test]
    #[serial]
    async fn test_spam_tasks() {
        dotenv().ok();
        init_logger(eigen_logging::log_level::LogLevel::Info);
        let data = &format!("{}", env!("CARGO_MANIFEST_DIR"));
        let mut path = Path::new(data);
        for _ in 0..4 {
            path = path
                .parent()
                .expect("Reached the filesystem root, no more parent directories");
        }

        let s = &format!(
            "{}/contracts/deployments/hello-world/31337.json",
            &path.display()
        );
        let data = std::fs::read_to_string(s).unwrap();
        let parsed: HelloWorldData = serde_json::from_str(&data).unwrap();
        let hello_world_contract_address: Address = parsed
            .addresses
            .hello_world_service_manager
            .parse()
            .unwrap();
        let provider = &get_provider(&ANVIL_RPC_URL);
        let hello_world_contract =
            HelloWorldServiceManager::new(hello_world_contract_address, provider);

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
