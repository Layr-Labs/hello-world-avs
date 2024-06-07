// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
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

contract HoleskyDeployer is Script, Utils {
    // ERC20 and Strategy: we need to deploy this erc20, create a strategy for it, and whitelist this strategy in the strategy manager

    ERC20Mock public erc20Mock;
    StrategyBase public erc20MockStrategy;

    // Hello World contracts
    ProxyAdmin public helloWorldProxyAdmin;
    PauserRegistry public helloWorldPauserReg;
    
    ECDSAStakeRegistry public stakeRegistryProxy;
    ECDSAStakeRegistry public stakeRegistryImplementation;

    HelloWorldServiceManager public helloWorldServiceManagerProxy;
    HelloWorldServiceManager public helloWorldServiceManagerImplementation;

    function run() external {
        // Manually pasted addresses of Eigenlayer contracts
        address strategyManagerAddr = 0xdfB5f6CE42aAA7830E94ECFCcAd411beF4d4D5b6;
        address delegationManagerAddr = 0xA44151489861Fe9e3055d95adC98FbD462B948e7;
        address avsDirectoryAddr = 0x055733000064333CaDDbC92763c58BF0192fFeBf;
        address eigenLayerProxyAdminAddr = 0xDB023566064246399b4AE851197a97729C93A6cf;
        address eigenLayerPauserRegAddr = 0x85Ef7299F8311B25642679edBF02B62FA2212F06;
        address baseStrategyImplementationAddr = 0x80528D6e9A2BAbFc766965E0E26d5aB08D9CFaF9;

        IStrategyManager strategyManager = IStrategyManager(strategyManagerAddr);
        IDelegationManager delegationManager = IDelegationManager(delegationManagerAddr);
        IAVSDirectory avsDirectory = IAVSDirectory(avsDirectoryAddr);
        ProxyAdmin eigenLayerProxyAdmin = ProxyAdmin(eigenLayerProxyAdminAddr);
        PauserRegistry eigenLayerPauserReg = PauserRegistry(eigenLayerPauserRegAddr);
        StrategyBase baseStrategyImplementation = StrategyBase(baseStrategyImplementationAddr);

        address helloWorldCommunityMultisig = msg.sender;
        address helloWorldPauser = msg.sender;

        vm.startBroadcast();
        _deployHelloWorldContracts(
            delegationManager,
            avsDirectory,
            baseStrategyImplementation,
            helloWorldCommunityMultisig,
            helloWorldPauser
        );
        vm.stopBroadcast();
    }

    function _deployHelloWorldContracts(
        IDelegationManager delegationManager,
        IAVSDirectory avsDirectory,
        IStrategy baseStrategyImplementation,
        address helloWorldCommunityMultisig,
        address helloWorldPauser
    ) internal {
        // Deploy proxy admin for ability to upgrade proxy contracts
        helloWorldProxyAdmin = new ProxyAdmin();

        // Deploy pauser registry
        {
            address[] memory pausers = new address[](2);
            pausers[0] = helloWorldPauser;
            pausers[1] = helloWorldCommunityMultisig;
            helloWorldPauserReg = new PauserRegistry(
                pausers,
                helloWorldCommunityMultisig
            );
        }

        EmptyContract emptyContract = new EmptyContract();

        // First, deploy upgradeable proxy contracts that will point to the implementations.
        helloWorldServiceManagerProxy = HelloWorldServiceManager(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(helloWorldProxyAdmin),
                    ""
                )
            )
        );
        stakeRegistryProxy = ECDSAStakeRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(helloWorldProxyAdmin),
                    ""
                )
            )
        );

        // Second, deploy the implementation contracts, using the proxy contracts as inputs
        {
            stakeRegistryImplementation = new ECDSAStakeRegistry(
                delegationManager
            );

            helloWorldProxyAdmin.upgrade(
                TransparentUpgradeableProxy(payable(address(stakeRegistryProxy))),
                address(stakeRegistryImplementation)
            );
        }

        {   
            // Create an array with one StrategyParams element
            StrategyParams memory strategyParams = StrategyParams({
                strategy: baseStrategyImplementation,
                multiplier: 10_000
            });

            StrategyParams[] memory quorumsStrategyParams = new StrategyParams[](1);
            quorumsStrategyParams[0] = strategyParams;

            Quorum memory quorum = Quorum(
                quorumsStrategyParams
            );

            // Sort the array (though it has only one element, it's trivially sorted)
            // If the array had more elements, you would need to ensure it is sorted by strategy address

            helloWorldProxyAdmin.upgradeAndCall(
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
        }

        helloWorldServiceManagerImplementation = new HelloWorldServiceManager(
            address(avsDirectory),
            address(stakeRegistryProxy),
            address(delegationManager)
        );
        // Upgrade the proxy contracts to use the correct implementation contracts and initialize them.
        helloWorldProxyAdmin.upgrade(
            TransparentUpgradeableProxy(
                payable(address(helloWorldServiceManagerProxy))
            ),
            address(helloWorldServiceManagerImplementation)
        );

        // WRITE JSON DATA
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(
            deployed_addresses,
            "HelloWorldServiceManagerProxy",
            address(helloWorldServiceManagerProxy)
        );
        vm.serializeAddress(
            deployed_addresses,
            "HelloWorldServiceManagerImplementation",
            address(helloWorldServiceManagerImplementation)
        );
        vm.serializeAddress(
            deployed_addresses,
            "ECDSAStakeRegistry",
            address(stakeRegistryProxy)
        );
        
        string memory deployed_addresses_output = vm.serializeAddress(
            deployed_addresses,
            "ECDSAStakeRegistryImplementation",
            address(stakeRegistryImplementation)
        );

        // Serialize all the data
        string memory finalJson = vm.serializeString(
            parent_object,
            deployed_addresses,
            deployed_addresses_output
        );

        writeOutput(finalJson, "hello_world_avs_holesky_deployment_output");
    }
}
