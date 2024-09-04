// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from
    "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {
    Quorum,
    StrategyParams
} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {HelloWorldServiceManager} from "../src/HelloWorldServiceManager.sol";
import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/Test.sol";
import {Utils} from "./utils/Utils.sol";
import {HelloWorldDeploymentLib} from "./utils/HelloWorldDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

// # To deploy and verify our contract
// forge script script/HelloWorldDeployer.s.sol:HelloWorldDeployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv
contract HelloWorldDeployer is Script, Utils {
    address private deployer;

    // ERC20 and Strategy: we need to deploy this erc20, create a strategy for it, and whitelist this strategy in the strategy manager
    address internal erc20Mock;
    address internal erc20MockStrategy;
    address internal delegationManager;
    address internal avsDirectory;
    address internal wethStrategy;

    // Hello World contracts
    address internal proxyAdmin;
    address internal helloWorldPauserReg;
    address internal stakeRegistry;
    address internal stakeRegistryImpl;
    address internal helloWorldServiceManager;
    address internal helloWorldServiceManagerImpl;

    StrategyParams internal strategyParams;
    Quorum internal quorum;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");

        delegationManager = 0xA44151489861Fe9e3055d95adC98FbD462B948e7;
        vm.label(delegationManager, "DelegationManager");

        avsDirectory = 0x055733000064333CaDDbC92763c58BF0192fFeBf; // TODO: need to get the actual anvil deployed address
        vm.label(avsDirectory, "AVSDirectory");

        wethStrategy = 0x80528D6e9A2BAbFc766965E0E26d5aB08D9CFaF9;
        vm.label(wethStrategy, "WETHStrategy");

        strategyParams = StrategyParams({strategy: IStrategy(wethStrategy), multiplier: 10_000});
        quorum.strategies.push(strategyParams);
    }

    function run() external {
        vm.startBroadcast(deployer);

        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        CoreDeploymentLib.DeploymentData memory core;
        HelloWorldDeploymentLib.DeploymentData memory result =
            HelloWorldDeploymentLib.deployContracts(address(proxyAdmin), core, quorum);

        helloWorldServiceManager = result.helloWorldServiceManager;
        stakeRegistry = result.stakeRegistry;

        vm.stopBroadcast();

        verifyDeployment();
        HelloWorldDeploymentLib.writeDeploymentJson(result);
    }

    function verifyDeployment() internal view {
        require(stakeRegistry != address(0), "StakeRegistry address cannot be zero");
        require(
            helloWorldServiceManager != address(0),
            "HelloWorldServiceManager address cannot be zero"
        );
        require(proxyAdmin != address(0), "ProxyAdmin address cannot be zero");
        require(delegationManager != address(0), "DelegationManager address cannot be zero");
        require(avsDirectory != address(0), "AVSDirectory address cannot be zero");
        require(wethStrategy != address(0), "WETHStrategy address cannot be zero");
    }
}
