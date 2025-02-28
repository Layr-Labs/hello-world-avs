#![cfg(not(doctest))]
#[allow(clippy::all, clippy::pedantic, unused_imports)]
pub mod helloworldservicemanager;

#[allow(clippy::all, clippy::pedantic, unused_imports)]
pub mod ecdsastakeregistry;

use std::path::Path;

use alloy::primitives::Address;
use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct HelloWorldData {
    #[serde(rename = "lastUpdate")]
    last_update: LastUpdate,
    pub addresses: HelloWorldAddresses,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct LastUpdate {
    timestamp: String,
    block_number: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct HelloWorldAddresses {
    #[serde(rename = "proxyAdmin")]
    _proxy_admin: String,
    #[serde(rename = "helloWorldServiceManager")]
    pub hello_world_service_manager: String,
    #[serde(rename = "helloWorldServiceManagerImpl")]
    hello_world_service_manager_impl: String,
    #[serde(rename = "stakeRegistry")]
    pub stake_registry: String,
    #[serde(rename = "stakeRegistryImpl")]
    stake_registry_impl: String,
    strategy: String,
    token: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct EigenLayerData {
    #[serde(rename = "lastUpdate")]
    last_update: LastUpdate,
    pub addresses: EigenLayerAddresses,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct EigenLayerAddresses {
    #[serde(rename = "proxyAdmin")]
    proxy_admin: String,
    pub delegation: String,
    #[serde(rename = "delegationManagerImpl")]
    delegation_manager_impl: String,
    #[serde(rename = "avsDirectory")]
    pub avs_directory: String,
    #[serde(rename = "avsDirectoryImpl")]
    avs_directory_impl: String,
    #[serde(rename = "strategyManager")]
    strategy_manager: String,
    #[serde(rename = "strategyManagerImpl")]
    strategy_manager_impl: String,
    #[serde(rename = "eigenPodManager")]
    eigen_pod_manager: String,
    #[serde(rename = "eigenPodManagerImpl")]
    eigen_pod_manager_impl: String,
    #[serde(rename = "strategyFactory")]
    strategy_factory: String,
    #[serde(rename = "strategyFactoryImpl")]
    strategy_factory_impl: String,
    #[serde(rename = "strategyBeacon")]
    strategy_beacon: String,
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
    let delegation_manager_address: Address = data.addresses.delegation.parse()?;
    Ok(delegation_manager_address)
}

pub fn get_avs_directory_address() -> eyre::Result<Address> {
    let data = get_anvil_eigenlayer_deployment_data()?;
    let avs_directory_address: Address = data.addresses.avs_directory.parse()?;
    Ok(avs_directory_address)
}
