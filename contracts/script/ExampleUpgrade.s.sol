// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/Test.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";
import {HelloWorldDeploymentLib} from "./utils/HelloWorldDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {HelloWorldServiceManager} from "../src/HelloWorldServiceManager.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";

contract ExampleUpgrade is Script {
    using UpgradeableProxyLib for address;
    using CoreDeploymentLib for *;

    address private deployer;
    address proxyAdmin;
    HelloWorldDeploymentLib.DeploymentData helloWorldDeployment;
    CoreDeploymentLib.DeploymentData coreDeployment;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");

        // Read existing deployments based on chain id
        coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);
        helloWorldDeployment = HelloWorldDeploymentLib.readDeploymentJson(block.chainid);
        proxyAdmin = helloWorldDeployment.proxyAdmin;
    }

    function run() external {
        vm.startBroadcast(deployer);

        /// Deploy new implementation contracts
        address newStakeRegistryImpl = address(
            new ECDSAStakeRegistry(IDelegationManager(coreDeployment.delegationManager))
        );

        address newHelloWorldServiceManagerImpl = address(
            new HelloWorldServiceManager(
                coreDeployment.avsDirectory,
                helloWorldDeployment.stakeRegistry,
                coreDeployment.rewardsCoordinator,
                coreDeployment.delegationManager
            )
        );

        /// This is where you will actually perform your upgrade
        helloWorldDeployment.helloWorldServiceManager.upgrade(newHelloWorldServiceManagerImpl);
        helloWorldDeployment.stakeRegistry.upgrade(newStakeRegistryImpl);

        vm.stopBroadcast();

        verifyUpgrade();
        HelloWorldDeploymentLib.writeDeploymentJson(helloWorldDeployment);
    }

    function verifyUpgrade() internal view {
        require(
            helloWorldDeployment.helloWorldServiceManager.getImplementation() != address(0),
            "HelloWorldServiceManager implementation cannot be zero"
        );
        require(
            helloWorldDeployment.stakeRegistry.getImplementation() != address(0),
            "StakeRegistry implementation cannot be zero"
        );
    }
}
