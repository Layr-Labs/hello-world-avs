// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {HelloWorldServiceManager} from "../src/HelloWorldServiceManager.sol";
import {MockAVSDeployer} from "@eigenlayer-middleware/test/utils/MockAVSDeployer.sol";
import {Vm} from "forge-std/Vm.sol";
import {console2} from "forge-std/Test.sol";
import {HelloWorldDeploymentLib} from "../script/utils/HelloWorldDeploymentLib.sol";
import {CoreDeploymentLib} from "../script/utils/CoreDeploymentLib.sol";
import {UpgradeableProxyLib} from "../script/utils/UpgradeableProxyLib.sol";

import {
    Quorum,
    StrategyParams,
    IStrategy
} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";

contract HelloWorldTaskManagerSetup is MockAVSDeployer {
    Quorum internal quorum;

    struct Operator {
        Vm.Wallet key;
        Vm.Wallet signingKey;
    }

    struct TrafficGenerator {
        Vm.Wallet key;
    }

    Operator[] internal operators;
    TrafficGenerator internal generator;

    HelloWorldDeploymentLib.DeploymentData internal helloWorldDeployment;
    CoreDeploymentLib.DeploymentData internal coreDeployment;

    function setUp() public virtual {
        operators.push(
            Operator({
                key: vm.createWallet("operator"),
                signingKey: vm.createWallet("operator_signing_wallet")
            })
        );

        generator = TrafficGenerator({key: vm.createWallet("generator_wallet")});

        address proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        coreDeployment = CoreDeploymentLib.deployContracts(proxyAdmin);

        quorum.strategies.push(
            StrategyParams({strategy: IStrategy(address(420)), multiplier: 10_000})
        );

        /// TODO: Update to take in as input the core deployment data struct which will clean things up
        helloWorldDeployment =
            HelloWorldDeploymentLib.deployContracts(proxyAdmin, coreDeployment, quorum);
    }
}

contract HelloWorldServiceManagerTest is HelloWorldTaskManagerSetup {
    function testTrue() public {
        assertTrue(true);
    }

    // Add more tests here
}
