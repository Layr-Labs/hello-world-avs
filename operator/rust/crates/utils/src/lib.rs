#[allow(clippy::all, clippy::pedantic,unused_imports)]
pub mod helloworldservicemanager;

#[allow(clippy::all, clippy::pedantic,unused_imports)]
pub mod ecdsastakeregistry;

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
