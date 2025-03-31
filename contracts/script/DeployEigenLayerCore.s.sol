// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";

import {CoreDeployLib, CoreDeploymentParsingLib} from "./utils/CoreDeploymentParsingLib.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {StrategyManager} from "@eigenlayer/contracts/core/StrategyManager.sol";

import "forge-std/Test.sol";

contract DeployEigenLayerCore is Script, Test {
    using CoreDeployLib for *;
    using UpgradeableProxyLib for address;

    address internal deployer;
    address internal proxyAdmin;
    CoreDeployLib.DeploymentData internal deploymentData;
    CoreDeployLib.DeploymentConfigData internal configData;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
    }

    function run() external {
        vm.startBroadcast(deployer);
        //set the rewards updater to the deployer address for payment flow
        configData =
            CoreDeploymentParsingLib.readDeploymentConfigValues("config/core/", block.chainid);
        configData.rewardsCoordinator.rewardsUpdater = deployer;
        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        deploymentData = CoreDeployLib.deployContracts(proxyAdmin, configData);

        // TODO: the deployer lib should probably do this
        StrategyManager(deploymentData.strategyManager).setStrategyWhitelister(
            deploymentData.strategyFactory
        );
        vm.stopBroadcast();
        string memory deploymentPath = "deployments/core/";
        CoreDeploymentParsingLib.writeDeploymentJson(deploymentPath, block.chainid, deploymentData);
    }
}
