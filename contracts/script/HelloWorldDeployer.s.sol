// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {Quorum, StrategyParams} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {HelloWorldServiceManager} from "../src/HelloWorldServiceManager.sol";
import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/Test.sol";
import {Utils} from "./utils/Utils.sol";

// # To deploy and verify our contract
// forge script script/HelloWorldDeployer.s.sol:HelloWorldDeployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv
contract HelloWorldDeployer is Script, Utils {
    bytes32 internal constant IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;
    bytes32 internal constant ADMIN_SLOT=0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103;
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
        delegationManager = 0xA44151489861Fe9e3055d95adC98FbD462B948e7;
        avsDirectory = 0x055733000064333CaDDbC92763c58BF0192fFeBf;
        wethStrategy = 0x80528D6e9A2BAbFc766965E0E26d5aB08D9CFaF9;

        strategyParams = StrategyParams({
            strategy: IStrategy(wethStrategy),
            multiplier: 10_000
        });
        quorum.strategies.push(strategyParams);
    }

    function run() external {
        vm.startBroadcast(deployer);
        
        // Deploy proxy admin for ability to upgrade proxy contracts
        proxyAdmin = address(new ProxyAdmin());

        // First, deploy upgradeable proxy contracts that will point to the implementations.
        helloWorldServiceManager =  setUpEmptyProxy(proxyAdmin);
        stakeRegistry = setUpEmptyProxy(proxyAdmin);
  
        // Deploy the implementation contracts, using the proxy contracts as inputs
        stakeRegistryImpl = address(new ECDSAStakeRegistry(IDelegationManager(delegationManager)));
        helloWorldServiceManagerImpl = address(new HelloWorldServiceManager(avsDirectory, stakeRegistry, delegationManager));

        // Label contracts for easier debugging
        vm.label(deployer, "Deployer");
        vm.label(delegationManager, "DelegationManager");
        vm.label(avsDirectory, "AVSDirectory");
        vm.label(wethStrategy, "WETHStrategy");
        vm.label(proxyAdmin, "ProxyAdmin");
        vm.label(helloWorldServiceManager, "HelloWorldServiceManager");
        vm.label(stakeRegistry, "StakeRegistry");
        vm.label(stakeRegistryImpl, "StakeRegistry Implementation");
        vm.label(helloWorldServiceManagerImpl, "HelloWorldServiceManager Implementation");
        
        bytes memory upgradeCall = abi.encodeCall(ECDSAStakeRegistry.initialize, (helloWorldServiceManager, 1, quorum));
        upgrade(stakeRegistry, stakeRegistryImpl, upgradeCall);
        upgrade(helloWorldServiceManager, helloWorldServiceManagerImpl);
        vm.stopBroadcast();

        require(stakeRegistry != address(0), "StakeRegistry address cannot be zero");
        require(stakeRegistryImpl != address(0), "StakeRegistry implementation address cannot be zero");
        require(helloWorldServiceManager != address(0), "HelloWorldServiceManager address cannot be zero");
        require(helloWorldServiceManagerImpl != address(0), "HelloWorldServiceManager implementation address cannot be zero");
        require(proxyAdmin != address(0), "ProxyAdmin address cannot be zero");
        require(delegationManager != address(0), "DelegationManager address cannot be zero");
        require(avsDirectory != address(0), "AVSDirectory address cannot be zero");
        require(wethStrategy != address(0), "WETHStrategy address cannot be zero");
    }

    function upgrade(address proxy, address impl) internal {
        ProxyAdmin admin = getProxyAdmin(proxy);
        admin.upgrade(TransparentUpgradeableProxy(payable(proxy)), impl);
    }

    function upgrade(address proxy, address impl, bytes memory initData) internal {
        ProxyAdmin admin = getProxyAdmin(proxy);
        admin.upgradeAndCall(TransparentUpgradeableProxy(payable(proxy)), impl, initData);

    }

    function setUpEmptyProxy(address admin) internal returns (address){
        address emptyContract = address(new EmptyContract());
        return address(new TransparentUpgradeableProxy(emptyContract, admin, ""));
    }

    function getImplementation(address proxy) internal view returns (address) {
        bytes32 value = vm.load(proxy, IMPLEMENTATION_SLOT);
        return address(uint160(uint256(value)));
    }

    function getProxyAdmin(address proxy) internal view returns (ProxyAdmin) {
        bytes32 value = vm.load(proxy, ADMIN_SLOT);
        return ProxyAdmin(address(uint160(uint256(value))));
    }

    function _updateDeploymentJson() internal {
        // Write deployment artifacts
        string memory deploymentData = string("");
        string memory directoryPath = "script/deployments/";
        string memory fileName =
            string(abi.encodePacked(directoryPath, vm.toString(block.chainid), ".json"));

        // Create directory if it doesn't exist
        if (!vm.exists(directoryPath)) {
            vm.createDir(directoryPath, true);
        }

        vm.writeFile(fileName, deploymentData);
        console2.log("Deployment artifacts written to:", fileName);
    }

}