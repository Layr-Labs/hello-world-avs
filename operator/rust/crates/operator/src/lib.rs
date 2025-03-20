//! Start creating tasks and respond appropriately
//! testing utils
/// Challenger struct for monitoring task completions and performing slashing
pub mod challenger;
/// Create createNewTask at regular intervals with random task names
pub mod spam_tasks;
/// Register Operator and monitor for NewTaskCreated event
pub mod start_operator;

/// Anvil container for testing
#[cfg(test)]
pub mod anvil;

#[cfg(test)]
mod tests {
    use crate::anvil::start_anvil_container;
    use crate::spam_tasks::create_new_task;
    use crate::start_operator::register_operator;

    use alloy::network::EthereumWallet;
    use alloy::primitives::Address;
    use alloy::providers::ProviderBuilder;
    use alloy::signers::local::PrivateKeySigner;
    use dotenv::dotenv;
    use eigensdk::common::get_provider;
    use eigensdk::logging::init_logger;

    use eigensdk::utils::slashing::core::delegationmanager::DelegationManager;
    use hello_world_utils::helloworldservicemanager::HelloWorldServiceManager::{
        self, latestTaskNumReturn,
    };
    use hello_world_utils::{
        get_anvil_eigenlayer_deployment_data, get_anvil_hello_world_deployment_data,
    };
    use reqwest::Url;
    use serial_test::serial;
    use std::env;
    use std::str::FromStr;
    use std::sync::LazyLock;

    static KEY: LazyLock<String> =
        LazyLock::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

    #[tokio::test]
    #[serial]
    async fn test_register_operator() {
        let (_container, anvil_http, _) = start_anvil_container().await;

        dotenv().ok();
        init_logger(eigensdk::logging::log_level::LogLevel::Info);
        let private_key = &KEY.clone();
        register_operator(&anvil_http, private_key).await.unwrap();

        let signer = PrivateKeySigner::from_str(private_key).unwrap();
        let wallet = EthereumWallet::from(signer.clone());
        let pr = ProviderBuilder::new()
            .wallet(wallet)
            .on_http(Url::from_str(&anvil_http).unwrap());
        let el_data = get_anvil_eigenlayer_deployment_data().unwrap();
        let delegation_manager_address: Address =
            el_data.addresses.delegation_manager.parse().unwrap();
        let contract_delegation_manager = DelegationManager::new(delegation_manager_address, &pr);

        let is_operator = contract_delegation_manager
            .isOperator(signer.address())
            .call()
            .await
            .unwrap()
            ._0;

        assert!(is_operator);
    }

    #[tokio::test]
    #[serial]
    async fn test_spam_tasks() {
        let (_container, anvil_http, _) = start_anvil_container().await;

        dotenv().ok();
        init_logger(eigensdk::logging::log_level::LogLevel::Info);

        let hw_data = get_anvil_hello_world_deployment_data().unwrap();
        let hello_world_contract_address: Address = hw_data
            .addresses
            .hello_world_service_manager
            .parse()
            .unwrap();
        let provider = &get_provider(&anvil_http);
        let hello_world_contract =
            HelloWorldServiceManager::new(hello_world_contract_address, provider);

        let latest_task_num = hello_world_contract.latestTaskNum().call().await.unwrap();

        let latestTaskNumReturn { _0: task_num } = latest_task_num;
        let _ = create_new_task(&anvil_http, "HelloEigen").await;

        let latest_task_num_after_creating_task =
            hello_world_contract.latestTaskNum().call().await.unwrap();
        let latestTaskNumReturn {
            _0: task_num_after_task,
        } = latest_task_num_after_creating_task;

        assert_eq!(task_num + 1, task_num_after_task);
    }
}
