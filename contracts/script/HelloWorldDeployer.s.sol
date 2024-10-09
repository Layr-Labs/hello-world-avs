// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/Test.sol";
import {HelloWorldDeploymentLib} from "./utils/HelloWorldDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

import {
    Quorum,
    StrategyParams,
    IStrategy
} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";

contract HelloWorldDeployer is Script {
    using CoreDeploymentLib for *;
    using UpgradeableProxyLib for address;

    address private deployer;
    address proxyAdmin;
    address aggregator;
    CoreDeploymentLib.DeploymentData coreDeployment;
    HelloWorldDeploymentLib.DeploymentData helloWorldDeployment;
    Quorum internal quorum;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("DEPLOYER_PRIVATE_KEY"));
        vm.label(deployer, "Deployer");

        coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);

        quorum.strategies.push(
            StrategyParams({strategy: IStrategy(address(420)), multiplier: 10_000})
        );
    }

    function run() external {
        vm.startBroadcast(deployer);
        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();

        aggregator = vm.addr(vm.envUint("AGGREGATOR_PRIVATE_KEY"));

        console2.log("aggregator", aggregator);

        helloWorldDeployment =
            HelloWorldDeploymentLib.deployContracts(proxyAdmin, aggregator, coreDeployment, quorum);

        vm.stopBroadcast();

        verifyDeployment();
        HelloWorldDeploymentLib.writeDeploymentJson(helloWorldDeployment);
    }

    function verifyDeployment() internal view {
        require(
            helloWorldDeployment.stakeRegistry != address(0), "StakeRegistry address cannot be zero"
        );
        require(
            helloWorldDeployment.helloWorldServiceManager != address(0),
            "HelloWorldServiceManager address cannot be zero"
        );
        require(proxyAdmin != address(0), "ProxyAdmin address cannot be zero");
        require(aggregator != address(0), "Aggregator address cannot be zero");
        require(
            coreDeployment.delegationManager != address(0),
            "DelegationManager address cannot be zero"
        );
        require(coreDeployment.avsDirectory != address(0), "AVSDirectory address cannot be zero");
    }
}
