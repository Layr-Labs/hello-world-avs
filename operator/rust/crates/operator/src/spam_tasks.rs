#![allow(missing_docs)]
use alloy_network::{Ethereum, EthereumSigner};
use alloy_primitives::Address;
use alloy_provider::fillers::{
    ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, SignerFiller,
};
use alloy_provider::{ProviderBuilder, RootProvider};
use alloy_signer_wallet::LocalWallet;
use alloy_sol_types::sol;
use alloy_transport_http::Client;
use dotenv::dotenv;
use eyre::Result;
use once_cell::sync::Lazy;
use rand::Rng;
use reqwest::Url;
use std::{env, str::FromStr};
use tokio::time::{self, Duration};
pub static RPC_URL: Lazy<String> =
    Lazy::new(|| env::var("RPC_URL").expect("failed to get rpc url from env"));

pub static HELLO_WORLD_CONTRACT_ADDRESS: Lazy<String> = Lazy::new(|| {
    env::var("CONTRACT_ADDRESS").expect("failed to get hello world contract address from env")
});
#[allow(unused)]
static KEY: Lazy<String> =
    Lazy::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    HelloWorldServiceManager,
    "json_abi/HelloWorldServiceManager.json"
);

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

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    dotenv().ok();
    start_creating_tasks().await;
}
