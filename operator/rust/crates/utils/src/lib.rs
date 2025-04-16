#![cfg(not(doctest))]
mod bindings;
use bindings::ecdsastakeregistry::{
    ECDSAStakeRegistry, ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry,
};
pub use bindings::*;
use chrono::Utc;
use eigensdk::{
    client_elcontracts::{
        reader::ELChainReader,
        writer::{ELChainWriter, Operator},
    },
    common::get_signer,
    logging::get_logger,
};
use rand::TryRngCore;

use std::{path::Path, str::FromStr};

use alloy::{
    primitives::{Address, FixedBytes, U256},
    providers::Provider,
    signers::{local::PrivateKeySigner, SignerSync},
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HelloWorldData {
    #[serde(rename = "lastUpdate")]
    pub last_update: LastUpdate,
    pub addresses: HelloWorldAddresses,
}

#[derive(Deserialize, Debug)]
pub struct LastUpdate {
    pub timestamp: String,
    pub block_number: String,
}

#[derive(Deserialize, Debug)]
pub struct HelloWorldAddresses {
    #[serde(rename = "proxyAdmin")]
    pub proxy_admin: String,
    #[serde(rename = "helloWorldServiceManager")]
    pub hello_world_service_manager: String,
    #[serde(rename = "helloWorldServiceManagerImpl")]
    pub hello_world_service_manager_impl: String,
    #[serde(rename = "stakeRegistry")]
    pub stake_registry: String,
    #[serde(rename = "stakeRegistryImpl")]
    pub stake_registry_impl: String,
    pub strategy: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct EigenLayerData {
    #[serde(rename = "lastUpdate")]
    pub last_update: LastUpdate,
    pub addresses: EigenLayerAddresses,
}

#[derive(Deserialize, Debug)]
pub struct EigenLayerAddresses {
    #[serde(rename = "proxyAdmin")]
    pub proxy_admin: String,
    #[serde(rename = "delegationManager")]
    pub delegation_manager: String,
    #[serde(rename = "delegationManagerImpl")]
    pub delegation_manager_impl: String,
    #[serde(rename = "avsDirectory")]
    pub avs_directory: String,
    #[serde(rename = "avsDirectoryImpl")]
    pub avs_directory_impl: String,
    #[serde(rename = "strategyManager")]
    pub strategy_manager: String,
    #[serde(rename = "strategyManagerImpl")]
    pub strategy_manager_impl: String,
    #[serde(rename = "eigenPodManager")]
    pub eigen_pod_manager: String,
    #[serde(rename = "eigenPodManagerImpl")]
    pub eigen_pod_manager_impl: String,
    #[serde(rename = "strategyFactory")]
    pub strategy_factory: String,
    #[serde(rename = "strategyFactoryImpl")]
    pub strategy_factory_impl: String,
    #[serde(rename = "strategyBeacon")]
    pub strategy_beacon: String,
}

pub fn get_anvil_eigenlayer_deployment_data() -> eyre::Result<EigenLayerData> {
    let file_path = Path::new(&env!("CARGO_MANIFEST_DIR").to_string())
        .join("../../../../contracts/deployments/core/31337.json");
    let data = std::fs::read_to_string(file_path)?;
    let el_parsed: EigenLayerData = serde_json::from_str(&data)?;
    Ok(el_parsed)
}

pub fn get_anvil_hello_world_deployment_data() -> eyre::Result<HelloWorldData> {
    let file_path = Path::new(&env!("CARGO_MANIFEST_DIR").to_string())
        .join("../../../../contracts/deployments/hello-world/31337.json");
    let data = std::fs::read_to_string(file_path)?;
    let parsed_data: HelloWorldData = serde_json::from_str(&data)?;
    Ok(parsed_data)
}

pub fn get_hello_world_service_manager() -> eyre::Result<Address> {
    let data = get_anvil_hello_world_deployment_data()?;
    let hello_world_contract_address: Address =
        data.addresses.hello_world_service_manager.parse()?;
    Ok(hello_world_contract_address)
}

pub fn get_stake_registry_address() -> eyre::Result<Address> {
    let data = get_anvil_hello_world_deployment_data()?;
    let stake_registry_address: Address = data.addresses.stake_registry.parse()?;
    Ok(stake_registry_address)
}

pub fn get_delegation_manager_address() -> eyre::Result<Address> {
    let data = get_anvil_eigenlayer_deployment_data()?;
    let delegation_manager_address: Address = data.addresses.delegation_manager.parse()?;
    Ok(delegation_manager_address)
}

pub fn get_avs_directory_address() -> eyre::Result<Address> {
    let data = get_anvil_eigenlayer_deployment_data()?;
    let avs_directory_address: Address = data.addresses.avs_directory.parse()?;
    Ok(avs_directory_address)
}

pub async fn register_operator(rpc_url: &str, private_key: &str) -> eyre::Result<()> {
    let pr = get_signer(private_key, rpc_url);
    let signer = PrivateKeySigner::from_str(private_key)?;

    let el_data = get_anvil_eigenlayer_deployment_data()?;
    let delegation_manager_address: Address = el_data.addresses.delegation_manager.parse()?;
    let avs_directory_address: Address = el_data.addresses.avs_directory.parse()?;

    let elcontracts_reader_instance = ELChainReader::new(
        get_logger().clone(),
        None,
        delegation_manager_address,
        Address::ZERO,
        avs_directory_address,
        None,
        rpc_url.to_string(),
    );
    let elcontracts_writer_instance = ELChainWriter::new(
        Address::ZERO,
        Address::ZERO,
        None,
        None,
        Address::ZERO,
        elcontracts_reader_instance.clone(),
        rpc_url.to_string(),
        private_key.to_string(),
    );

    let operator = Operator {
        address: signer.address(),
        delegation_approver_address: Address::ZERO,
        staker_opt_out_window_blocks: Some(0),
        metadata_url: Default::default(),
        allocation_delay: Some(0),
        _deprecated_earnings_receiver_address: None,
    };

    let is_registered = elcontracts_reader_instance
        .is_operator_registered(signer.address())
        .await
        .unwrap();
    get_logger().info(&format!("is registered {}", is_registered), "");
    let tx_hash = elcontracts_writer_instance
        .register_as_operator(operator)
        .await?;
    let receipt = pr.get_transaction_receipt(tx_hash).await?;
    if !receipt.is_some_and(|r| r.inner.is_success()) {
        get_logger().error("Operator registration failed", "");
        return Err(eyre::eyre!("Operator registration failed"));
    }
    get_logger().info(
        &format!(
            "Operator registered on EL successfully tx_hash {:?}",
            tx_hash
        ),
        "",
    );
    let mut salt = [0u8; 32];
    rand::rngs::OsRng.try_fill_bytes(&mut salt).unwrap();

    let salt = FixedBytes::from_slice(&salt);
    let now = Utc::now().timestamp();
    let expiry = U256::from(now + 3600);

    let hello_world_contract_address: Address = get_hello_world_service_manager()?;
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
        expiry,
    };
    let stake_registry_address = get_stake_registry_address()?;
    let contract_ecdsa_stake_registry = ECDSAStakeRegistry::new(stake_registry_address, &pr);
    let registeroperator_details_call = contract_ecdsa_stake_registry
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
        "",
    );

    Ok(())
}
