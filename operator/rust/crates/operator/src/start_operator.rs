#![allow(missing_docs)]
use alloy::dyn_abi::DynSolValue;
use alloy::{
    primitives::{eip191_hash_message, keccak256, Address, FixedBytes, U256},
    providers::Provider,
    rpc::types::{BlockNumberOrTag, Filter},
    signers::{local::PrivateKeySigner, SignerSync},
    sol_types::{SolEvent, SolValue},
};
use chrono::Utc;
use dotenv::dotenv;
use eigen_client_elcontracts::{
    reader::ELChainReader,
    writer::{ELChainWriter, Operator},
};
use eigen_logging::{get_logger, init_logger, log_level::LogLevel};
use eigen_utils::{get_provider, get_signer};
use eyre::Result;
use hello_world_utils::ecdsastakeregistry::ECDSAStakeRegistry;
use hello_world_utils::{
    ecdsastakeregistry::ISignatureUtils::SignatureWithSaltAndExpiry,
    helloworldservicemanager::{HelloWorldServiceManager, IHelloWorldServiceManager::Task},
};
use hello_world_utils::{
    parse_hello_world_service_manager, parse_stake_registry_address, EigenLayerData,
};
use once_cell::sync::Lazy;
use rand::RngCore;
use std::{env, str::FromStr};

pub const ANVIL_RPC_URL: &str = "http://localhost:8545";

static KEY: Lazy<String> =
    Lazy::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

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

    get_logger().info(
        &format!("Signing and responding to task : {:?}", task_index),
        "",
    );
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
    get_logger().info(
        &format!("Responded to task with tx hash {}", response_hash),
        "",
    );
    Ok(())
}

/// Monitor new tasks
async fn monitor_new_tasks() -> Result<()> {
    let pr = get_signer(&KEY.clone(), ANVIL_RPC_URL);
    let hello_world_contract_address: Address =
        parse_hello_world_service_manager("contracts/deployments/hello-world/31337.json")?;
    let mut latest_processed_block = pr.get_block_number().await?;

    loop {
        get_logger().info("Monitoring for new tasks...", "");

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
                    get_logger().info(&format!("New task detected :Hello{:?} ", task.name), "");

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
    let pr = get_signer(&KEY.clone(), ANVIL_RPC_URL);
    let signer = PrivateKeySigner::from_str(&KEY.clone())?;

    let default_slasher = Address::ZERO; // We don't need slasher for our example.
    let default_strategy = Address::ZERO; // We don't need strategy for our example.

    let data = std::fs::read_to_string("contracts/deployments/core/31337.json")?;
    let el_parsed: EigenLayerData = serde_json::from_str(&data)?;
    let delegation_manager_address: Address = el_parsed.addresses.delegation.parse()?;
    let avs_directory_address: Address = el_parsed.addresses.avs_directory.parse()?;

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
        &format!(
            "Operator registered on EL successfully tx_hash {:?}",
            tx_hash
        ),
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
    let stake_registry_address =
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

#[tokio::main]
pub async fn main() {
    use tokio::signal;
    dotenv().ok();
    init_logger(LogLevel::Info);
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

    // Wait for a Ctrl+C signal to gracefully shut down
    let _ = signal::ctrl_c().await;
    get_logger().info("Received Ctrl+C, shutting down...", "");
}
