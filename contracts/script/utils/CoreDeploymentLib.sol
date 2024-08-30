// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from
    "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {UpgradeableBeacon} from "@openzeppelin/contracts/proxy/beacon/UpgradeableBeacon.sol";
import {console2} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {DelegationManager} from "@eigenlayer/contracts/core/DelegationManager.sol";
import {StrategyManager} from "@eigenlayer/contracts/core/StrategyManager.sol";
import {AVSDirectory} from "@eigenlayer/contracts/core/AVSDirectory.sol";
import {Slasher} from "@eigenlayer/contracts/core/Slasher.sol";
import {EigenPodManager} from "@eigenlayer/contracts/pods/EigenPodManager.sol";
import {RewardsCoordinator} from "@eigenlayer/contracts/core/RewardsCoordinator.sol";
import {StrategyBase} from "@eigenlayer/contracts/strategies/StrategyBase.sol";
import {EigenPod} from "@eigenlayer/contracts/pods/EigenPod.sol";
import {IETHPOSDeposit} from "@eigenlayer/contracts/interfaces/IETHPOSDeposit.sol";
import {StrategyBaseTVLLimits} from "@eigenlayer/contracts/strategies/StrategyBaseTVLLimits.sol";
import {PauserRegistry} from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategy.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ISignatureUtils} from "@eigenlayer/contracts/interfaces/ISignatureUtils.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IBeacon} from "@openzeppelin/contracts/proxy/beacon/IBeacon.sol";
import {IStrategyManager} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {ISlasher} from "@eigenlayer/contracts/interfaces/ISlasher.sol";
import {IEigenPodManager} from "@eigenlayer/contracts/interfaces/IEigenPodManager.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IPauserRegistry} from "@eigenlayer/contracts/interfaces/IPauserRegistry.sol";

import {UpgradeableProxyLib} from "./UpgradeableProxyLib.sol";

library CoreDeploymentLib {
    using stdJson for *;

    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    struct DeploymentData {
        address delegationManager;
        address avsDirectory;
        address wethStrategy;
        address strategyManager;
        address eigenPodManager;
        address rewardsCoordinator;
        address eigenPodBeacon;
        address pauserRegistry;
    }

    function deployContracts(
        address proxyAdmin
    ) internal returns (DeploymentData memory) {
        DeploymentData memory result;

        // First, deploy upgradeable proxy contracts that will point to the implementations.
        result.delegationManager = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        vm.label(result.delegationManager, "DelegationManager");

        result.avsDirectory = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        vm.label(result.avsDirectory, "AVSDirectory");
        result.strategyManager = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        vm.label(result.strategyManager, "StrategyManager");

        result.eigenPodManager = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        vm.label(result.eigenPodManager, "EigenPodManager");

        result.rewardsCoordinator = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        vm.label(result.rewardsCoordinator, "RewardsCoordinator");

        result.eigenPodBeacon = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        vm.label(result.eigenPodBeacon, "EigenPodBeacon");

        result.pauserRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        vm.label(result.pauserRegistry, "PauserRegistry");

        // Deploy the implementation contracts, using the proxy contracts as inputs
        address delegationManagerImpl;
        vm.label(delegationManagerImpl, "DelegationManager Implementation");
        address avsDirectoryImpl =
            address(new AVSDirectory(IDelegationManager(result.delegationManager)));
        vm.label(avsDirectoryImpl, "AVSDirectory Implementation");

        address strategyManagerImpl = address(
            new StrategyManager(
                IDelegationManager(result.delegationManager),
                IEigenPodManager(result.eigenPodManager),
                ISlasher(address(0))
            )
        );
        vm.label(strategyManagerImpl, "StrategyManager Implementation");

        address ethPOSDeposit;

        address eigenPodManagerImpl = address(
            new EigenPodManager(
                IETHPOSDeposit(ethPOSDeposit),
                IBeacon(result.eigenPodBeacon),
                IStrategyManager(result.strategyManager),
                ISlasher(address(0)),
                IDelegationManager(result.delegationManager)
            )
        );
        vm.label(eigenPodManagerImpl, "EigenPodManager Implementation");

        uint32 CALCULATION_INTERVAL_SECONDS;
        uint32 MAX_REWARDS_DURATION;
        uint32 MAX_RETROACTIVE_LENGTH;
        uint32 MAX_FUTURE_LENGTH;
        uint32 GENESIS_REWARDS_TIMESTAMP;
        address rewardsCoordinatorImpl = address(
            new RewardsCoordinator(
                IDelegationManager(result.delegationManager),
                IStrategyManager(result.strategyManager),
                CALCULATION_INTERVAL_SECONDS,
                MAX_REWARDS_DURATION,
                MAX_RETROACTIVE_LENGTH,
                MAX_FUTURE_LENGTH,
                GENESIS_REWARDS_TIMESTAMP
            )
        );
        vm.label(rewardsCoordinatorImpl, "RewardsCoordinator Implementation");

        uint64 GENESIS_TIME;
        address eigenPodImpl = address(
            new EigenPod(
                IETHPOSDeposit(ethPOSDeposit),
                IEigenPodManager(result.eigenPodManager),
                GENESIS_TIME
            )
        );
        vm.label(eigenPodImpl, "EigenPod Implementation");

        address eigenPodBeaconImpl = address(new UpgradeableBeacon(eigenPodImpl));
        vm.label(eigenPodBeaconImpl, "EigenPodBeacon Implementation");

        address baseStrategyImpl =
            address(new StrategyBaseTVLLimits(IStrategyManager(result.strategyManager)));
        vm.label(baseStrategyImpl, "BaseStrategy Implementation");

        address pauserRegistryImpl = address(
            new PauserRegistry(
                new address[](0), // Empty array for pausers
                proxyAdmin // ProxyAdmin as the unpauser
            )
        );
        vm.label(pauserRegistryImpl, "PauserRegistry Implementation");

        // Upgrade contracts
        bytes memory upgradeCall;
        //  = abi.encodeCall(DelegationManager.initialize, ());
        UpgradeableProxyLib.upgradeAndCall(
            result.delegationManager, delegationManagerImpl, upgradeCall
        );
        return result;
    }

    struct DeploymentConfigData {
        uint256 value;
    }

    function readDeploymentConfigValues(
        string memory directoryPath,
        uint256 chainId
    ) internal returns (DeploymentConfigData memory) {
        string memory fileName = string.concat(directoryPath, vm.toString(chainId), ".json");

        require(vm.exists(fileName), "Deployment file does not exist");

        string memory json = vm.readFile(fileName);

        DeploymentConfigData memory data;

        return data;
    }

    function readDeploymentJson(
        string memory directoryPath,
        uint256 chainId
    ) internal returns (DeploymentData memory) {
        string memory fileName = string.concat(directoryPath, vm.toString(chainId), ".json");

        require(vm.exists(fileName), "Deployment file does not exist");

        string memory json = vm.readFile(fileName);

        DeploymentData memory data;
        data.strategyManager = json.readAddress(".contracts.strategyManager");
        data.eigenPodManager = json.readAddress(".contracts.eigenPodManager");
        data.delegationManager = json.readAddress(".contracts.delegationManager");
        data.avsDirectory = json.readAddress(".contracts.avsDirectory");
        data.wethStrategy = json.readAddress(".contracts.wethStrategy");

        return data;
    }

    function writeDeploymentJson(
        DeploymentData memory data
    ) internal {
        address proxyAdmin = address(UpgradeableProxyLib.getProxyAdmin(data.strategyManager));

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
        /// TODO: namespace contracts -> {avs, core}
        return string.concat(
            '{"proxyAdmin":"',
            vm.toString(proxyAdmin),
            '","delegationManager":"',
            vm.toString(data.delegationManager),
            '","delegationManagerImpl":"',
            vm.toString(UpgradeableProxyLib.getImplementation(data.delegationManager)),
            '","avsDirectory":"',
            vm.toString(data.avsDirectory),
            '","avsDirectoryImpl":"',
            vm.toString(UpgradeableProxyLib.getImplementation(data.avsDirectory)),
            '","wethStrategy":"',
            vm.toString(data.wethStrategy),
            '","strategyManager":"',
            vm.toString(data.strategyManager),
            '","strategyManagerImpl":"',
            vm.toString(UpgradeableProxyLib.getImplementation(data.strategyManager)),
            '","eigenPodManager":"',
            vm.toString(data.eigenPodManager),
            '","eigenPodManagerImpl":"',
            vm.toString(UpgradeableProxyLib.getImplementation(data.eigenPodManager)),
            '"}'
        );
    }
}
