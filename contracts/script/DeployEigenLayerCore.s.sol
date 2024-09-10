// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import "forge-std/Script.sol";
import "forge-std/Test.sol";

import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

contract DeployEigenlayerCore is Script, Test {
    using CoreDeploymentLib for *;
    using UpgradeableProxyLib for address;

    address proxyAdmin;
    CoreDeploymentLib.DeploymentData deploymentData;
    CoreDeploymentLib.DeploymentConfigData configData;

    function setUp() public virtual {
        _setUpCoreContractsConfig();
    }

    function run() external {
        vm.startBroadcast();
        _deployCoreContracts();
        vm.stopBroadcast();
    }

    function _setUpCoreContractsConfig() internal {
        string memory configPath =
            "lib/eigenlayer-middleware/lib/eigenlayer-contracts/script/configs/devnet/";
        uint256 chainId = block.chainid;

        configData = CoreDeploymentLib.readDeploymentConfigValues(configPath, chainId);
    }

    function _deployCoreContracts() internal {
        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        deploymentData = CoreDeploymentLib.deployContracts(proxyAdmin, configData);

        // Write deployment data to JSON file
        string memory deploymentPath = "script/output/";
        CoreDeploymentLib.writeDeploymentJson(deploymentPath, block.chainid, deploymentData);
    }
}
