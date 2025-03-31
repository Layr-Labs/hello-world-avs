use std::path::{Path, PathBuf};
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
const ANVIL_IMAGE: &str = "ghcr.io/foundry-rs/foundry";
const ANVIL_TAG: &str = "latest";
const ANVIL_STATE_PATH: &str = "contracts/anvil/state.json";

fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

/// Start an anvil container for testing, using the dump state file `ANVIL_STATE_PATH`
pub async fn start_anvil_container() -> (ContainerAsync<GenericImage>, String, String) {
    let relative_path = PathBuf::from(ANVIL_STATE_PATH);
    let absolute_path = workspace_dir().join(relative_path);
    let absolute_path_str = absolute_path.to_str().unwrap();

    let container = GenericImage::new(ANVIL_IMAGE, ANVIL_TAG)
        .with_entrypoint("anvil")
        .with_wait_for(WaitFor::message_on_stdout("Listening on"))
        .with_exposed_port(8545.tcp())
        .with_mount(testcontainers::core::Mount::bind_mount(
            absolute_path_str,
            "/state.json",
        ))
        .with_cmd([
            "--host",
            "0.0.0.0",
            "--load-state",
            "/state.json",
            "--base-fee",
            "0",
            "--gas-price",
            "0",
            "--port",
            "8545",
        ])
        .start()
        .await
        .unwrap();

    let port = container
        .ports()
        .await
        .unwrap()
        .map_to_host_port_ipv4(8545.tcp())
        .unwrap();

    let http_endpoint = format!("http://localhost:{port}");
    let ws_endpoint = format!("ws://localhost:{port}");

    (container, http_endpoint, ws_endpoint)
}
