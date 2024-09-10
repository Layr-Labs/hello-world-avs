// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/Test.sol";
import {Utils} from "./utils/Utils.sol";
import {HelloWorldDeploymentLib} from "./utils/HelloWorldDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

import {
    Quorum,
    StrategyParams,
    IStrategy
} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";

// # To deploy and verify our contract
// forge script script/HelloWorldDeployer.s.sol:HelloWorldDeployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv
contract HelloWorldDeployer is Script, Utils {
    using CoreDeploymentLib for *;
    using UpgradeableProxyLib for address;

    address private deployer;
    address proxyAdmin;
    CoreDeploymentLib.DeploymentData coreDeployment;
    CoreDeploymentLib.DeploymentConfigData coreConfigData;
    HelloWorldDeploymentLib.DeploymentData helloWorldDeployment;
    Quorum internal quorum;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");

        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();

        coreConfigData = CoreDeploymentLib.readDeploymentConfigValues("config/core/", block.chainid);
        coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);

        quorum.strategies.push(
            StrategyParams({strategy: IStrategy(coreDeployment.wethStrategy), multiplier: 10_000})
        );
    }

    function run() external {
        vm.startBroadcast(deployer);

        helloWorldDeployment =
            HelloWorldDeploymentLib.deployContracts(proxyAdmin, coreDeployment, quorum);

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
        require(
            coreDeployment.delegationManager != address(0),
            "DelegationManager address cannot be zero"
        );
        require(coreDeployment.avsDirectory != address(0), "AVSDirectory address cannot be zero");
        require(coreDeployment.wethStrategy != address(0), "WETHStrategy address cannot be zero");
    }
}
