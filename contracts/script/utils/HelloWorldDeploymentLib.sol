// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from
    "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {HelloWorldServiceManager} from "../../src/HelloWorldServiceManager.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {Quorum} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {UpgradeableProxyLib} from "./UpgradeableProxyLib.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

library HelloWorldDeploymentLib {
    using stdJson for *;
    using Strings for *;
    using UpgradeableProxyLib for address;

    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    struct DeploymentData {
        address helloWorldServiceManager;
        address stakeRegistry;
        address delegationManager;
        address avsDirectory;
        address wethStrategy;
        address strategyManager;
        address eigenPodManager;
    }

    function deployContracts(
        address proxyAdmin,
        address delegationManager,
        address avsDirectory,
        Quorum memory quorum
    ) internal returns (DeploymentData memory) {
        DeploymentData memory result;

        // First, deploy upgradeable proxy contracts that will point to the implementations.
        result.helloWorldServiceManager = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        vm.label(result.helloWorldServiceManager, "HelloWorldServiceManager");

        result.stakeRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        vm.label(result.stakeRegistry, "StakeRegistry");

        // Deploy the implementation contracts, using the proxy contracts as inputs
        address stakeRegistryImpl =
            address(new ECDSAStakeRegistry(IDelegationManager(delegationManager)));
        vm.label(stakeRegistryImpl, "StakeRegistry Implementation");

        address helloWorldServiceManagerImpl = address(
            new HelloWorldServiceManager(avsDirectory, result.stakeRegistry, delegationManager)
        );
        vm.label(helloWorldServiceManagerImpl, "HelloWorldServiceManager Implementation");

        // Upgrade contracts
        bytes memory upgradeCall = abi.encodeCall(
            ECDSAStakeRegistry.initialize, (result.helloWorldServiceManager, 1, quorum)
        );
        UpgradeableProxyLib.upgradeAndCall(result.stakeRegistry, stakeRegistryImpl, upgradeCall);
        UpgradeableProxyLib.upgrade(result.helloWorldServiceManager, helloWorldServiceManagerImpl);

        result.avsDirectory = avsDirectory;
        result.delegationManager = delegationManager;
        result.wethStrategy = address(quorum.strategies[0].strategy);

        return result;
    }

    function readDeploymentJson(
        uint256 chainId
    ) internal returns (DeploymentData memory) {
        /// TODO: Parameterize this
        string memory directoryPath = "deployments/";
        string memory fileName = string.concat(directoryPath, vm.toString(chainId), ".json");

        require(vm.exists(fileName), "Deployment file does not exist");

        string memory json = vm.readFile(fileName);

        DeploymentData memory data;
        data.strategyManager = json.readAddress(".contracts.strategyManager");
        data.eigenPodManager = json.readAddress(".contracts.eigenPodManager");
        data.helloWorldServiceManager = json.readAddress(".contracts.helloWorldServiceManager");
        data.stakeRegistry = json.readAddress(".contracts.stakeRegistry");
        data.delegationManager = json.readAddress(".contracts.delegationManager");
        data.avsDirectory = json.readAddress(".contracts.avsDirectory");
        data.wethStrategy = json.readAddress(".contracts.wethStrategy");

        return data;
    }

    function writeDeploymentJson(
        DeploymentData memory data
    ) internal {
        address proxyAdmin =
            address(UpgradeableProxyLib.getProxyAdmin(data.helloWorldServiceManager));

        string memory deploymentData = _generateDeploymentJson(data, proxyAdmin);

        string memory directoryPath = "deployments/";
        string memory fileName = string.concat(directoryPath, vm.toString(block.chainid), ".json");
        if (!vm.exists(directoryPath)) {
            vm.createDir(directoryPath, true);
        }

        vm.writeFile(fileName, deploymentData);
        console2.log("Deployment artifacts written to:", fileName);
    }

    function _generateDeploymentJson(
        DeploymentData memory data,
        address proxyAdmin
    ) private view returns (string memory) {
        return string.concat(
            '{"lastUpdate":{"timestamp":"',
            vm.toString(block.timestamp),
            '","block_number":"',
            vm.toString(block.number),
            '"},"contracts":',
            _generateContractsJson(data, proxyAdmin),
            "}"
        );
    }

    function _generateContractsJson(
        DeploymentData memory data,
        address proxyAdmin
    ) private view returns (string memory) {
        return string.concat(
            '{"proxyAdmin":"',
            proxyAdmin.toHexString(),
            '","helloWorldServiceManager":"',
            data.helloWorldServiceManager.toHexString(),
            '","helloWorldServiceManagerImpl":"',
            data.helloWorldServiceManager.getImplementation().toHexString(),
            '","stakeRegistry":"',
            data.stakeRegistry.toHexString(),
            '","stakeRegistryImpl":"',
            data.stakeRegistry.getImplementation().toHexString(),
            '","delegationManager":"',
            data.delegationManager.toHexString(),
            '","delegationManagerImpl":"',
            data.delegationManager.getImplementation().toHexString(),
            '","avsDirectory":"',
            data.avsDirectory.toHexString(),
            '","avsDirectoryImpl":"',
            data.avsDirectory.getImplementation().toHexString(),
            '","wethStrategy":"',
            data.wethStrategy.toHexString(),
            '","strategyManager":"',
            data.strategyManager.toHexString(),
            '","strategyManagerImpl":"',
            data.strategyManager.getImplementation().toHexString(),
            '","eigenPodManager":"',
            data.eigenPodManager.toHexString(),
            '","eigenPodManagerImpl":"',
            data.eigenPodManager.getImplementation().toHexString(),
            '"}'
        );
    }
}
