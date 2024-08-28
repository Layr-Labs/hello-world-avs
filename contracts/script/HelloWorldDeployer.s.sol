// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IStrategyManager, IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {StrategyBase} from "@eigenlayer/contracts/strategies/StrategyBase.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {Quorum, StrategyParams} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {HelloWorldServiceManager} from "../src/HelloWorldServiceManager.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";
import "../src/ERC20Mock.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";
import {Utils} from "./utils/Utils.sol";

// # To deploy and verify our contract
// forge script script/HelloWorldDeployer.s.sol:HelloWorldDeployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv
contract HelloWorldDeployer is Script, Utils {
   // ERC20 and Strategy: we need to deploy this erc20, create a strategy for it, and whitelist this strategy in the strategy manager

    ERC20Mock public erc20Mock;
    StrategyBase public erc20MockStrategy;

    // Hello World contracts
    ProxyAdmin public proxyAdmin;
    PauserRegistry public helloWorldPauserReg;
    
    ECDSAStakeRegistry public stakeRegistryProxy;
    ECDSAStakeRegistry public stakeRegistryImplementation;

    HelloWorldServiceManager public helloWorldServiceManagerProxy;
    HelloWorldServiceManager public helloWorldServiceManagerImplementation;

    function run() external {
        // Manually pasted addresses of Eigenlayer contracts from https://github.com/Layr-Labs/eigenlayer-contracts/?tab=readme-ov-file#current-testnet-deployment
        address delegationManagerProxyAddr = 0xA44151489861Fe9e3055d95adC98FbD462B948e7;
        address avsDirectoryProxyAddr = 0x055733000064333CaDDbC92763c58BF0192fFeBf;
        address wethStrategyProxyAddr = 0x80528D6e9A2BAbFc766965E0E26d5aB08D9CFaF9;

        IDelegationManager delegationManager = IDelegationManager(delegationManagerProxyAddr);
        IAVSDirectory avsDirectory = IAVSDirectory(avsDirectoryProxyAddr);
        StrategyBase wethStrategy = StrategyBase(wethStrategyProxyAddr);

        vm.startBroadcast();
        
        
 // Deploy proxy admin for ability to upgrade proxy contracts
        proxyAdmin = new ProxyAdmin();

        EmptyContract emptyContract = new EmptyContract();

        // First, deploy upgradeable proxy contracts that will point to the implementations.
        helloWorldServiceManagerProxy = HelloWorldServiceManager(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(proxyAdmin),
                    ""
                )
            )
        );

        stakeRegistryProxy = ECDSAStakeRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(proxyAdmin),
                    ""
                )
            )
        );
  
        // Deploy the implementation contracts, using the proxy contracts as inputs
        stakeRegistryImplementation = new ECDSAStakeRegistry(
            delegationManager
        );
        
        // Upgrade stake registry proxy to point to the implementation
        proxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(stakeRegistryProxy))),
            address(stakeRegistryImplementation)
        );
        
        // Build quorum struct
        StrategyParams memory strategyParams = StrategyParams({
            strategy: wethStrategy,
            multiplier: 10_000
        });
        StrategyParams[] memory quorumsStrategyParams = new StrategyParams[](1);
        quorumsStrategyParams[0] = strategyParams;
        Quorum memory quorum = Quorum(
            quorumsStrategyParams
        );

        // Upgrade HelloWorldServiceManager contract
        proxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(
                payable(address(stakeRegistryProxy))
            ),
            address(stakeRegistryImplementation),
            abi.encodeWithSelector(
                ECDSAStakeRegistry.initialize.selector,
                address(helloWorldServiceManagerProxy),
                1,
                quorum
            )
        );
        
        
        helloWorldServiceManagerImplementation = new HelloWorldServiceManager(
            address(avsDirectory),
            address(stakeRegistryProxy),
            address(delegationManager)
        );
        // Upgrade the proxy contracts to use the correct implementation contracts and initialize them.
        proxyAdmin.upgrade(
            TransparentUpgradeableProxy(
                payable(address(helloWorldServiceManagerProxy))
            ),
            address(helloWorldServiceManagerImplementation)
        );
        vm.stopBroadcast();
    }
       
}