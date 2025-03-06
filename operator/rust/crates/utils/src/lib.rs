#![cfg(not(doctest))]
mod bindings;
pub use bindings::*;

use std::path::Path;

use alloy::primitives::Address;
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
