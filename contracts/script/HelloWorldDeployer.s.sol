// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IStrategyManager, IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {StrategyBaseTVLLimits} from "@eigenlayer/contracts/strategies/StrategyBaseTVLLimits.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";
import {RegistryCoordinator, IRegistryCoordinator, IBLSApkRegistry, IIndexRegistry, IStakeRegistry} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {BLSApkRegistry} from "@eigenlayer-middleware/src/BLSApkRegistry.sol";
import {IndexRegistry} from "@eigenlayer-middleware/src/IndexRegistry.sol";
import {StakeRegistry} from "@eigenlayer-middleware/src/StakeRegistry.sol";
import "@eigenlayer-middleware/src/OperatorStateRetriever.sol";
import {HelloWorldServiceManager, IServiceManager} from "../src/HelloWorldServiceManager.sol";
import "../src/ERC20Mock.sol";
import "forge-std/Test.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";

// # To deploy and verify our contract
// forge script script/HelloWorldDeployer.s.sol:HelloWorldDeployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv
contract HelloWorldDeployer is Script {
    address public constant AGGREGATOR_ADDR = 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720;
    address public constant TASK_GENERATOR_ADDR = 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720;

    ERC20Mock public erc20Mock;
    StrategyBaseTVLLimits public erc20MockStrategy;

    ProxyAdmin public helloWorldProxyAdmin;
    PauserRegistry public helloWorldPauserReg;

    RegistryCoordinator public registryCoordinator;
    IStakeRegistry public stakeRegistry;

    HelloWorldServiceManager public helloWorldServiceManager;

    function run() external {
        string memory eigenlayerDeployedContracts = readOutput("eigenlayer_deployment_output");
        IStrategyManager strategyManager = IStrategyManager(
            stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.strategyManager")
        );
        IDelegationManager delegationManager = IDelegationManager(
            stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.delegation")
        );
        IAVSDirectory avsDirectory = IAVSDirectory(
            stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.avsDirectory")
        );
        ProxyAdmin eigenLayerProxyAdmin = ProxyAdmin(
            stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.eigenLayerProxyAdmin")
        );
        PauserRegistry eigenLayerPauserReg = PauserRegistry(
            stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.eigenLayerPauserReg")
        );
        StrategyBaseTVLLimits baseStrategyImplementation = StrategyBaseTVLLimits(
            stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.baseStrategyImplementation")
        );

        address helloWorldCommunityMultisig = msg.sender;
        address helloWorldPauser = msg.sender;

        vm.startBroadcast();
        _deployErc20AndStrategyAndWhitelistStrategy(
            eigenLayerProxyAdmin,
            eigenLayerPauserReg,
            baseStrategyImplementation,
            strategyManager
        );
        _deployHelloWorldContracts(
            delegationManager,
            avsDirectory,
            erc20MockStrategy,
            helloWorldCommunityMultisig,
            helloWorldPauser
        );
        vm.stopBroadcast();
    }

    function _deployErc20AndStrategyAndWhitelistStrategy(
        ProxyAdmin eigenLayerProxyAdmin,
        PauserRegistry eigenLayerPauserReg,
        StrategyBaseTVLLimits baseStrategyImplementation,
        IStrategyManager strategyManager
    ) internal {
        erc20Mock = new ERC20Mock();
        erc20MockStrategy = StrategyBaseTVLLimits(
            address(
                new TransparentUpgradeableProxy(
                    address(baseStrategyImplementation),
                    address(eigenLayerProxyAdmin),
                    abi.encodeWithSelector(
                        StrategyBaseTVLLimits.initialize.selector,
                        1 ether, // maxPerDeposit
                        100 ether, // maxDeposits
                        IERC20(erc20Mock),
                        eigenLayerPauserReg
                    )
                )
            )
        );
        IStrategy[] memory strats = new IStrategy[](1);
        strats[0] = erc20MockStrategy;
        bool[] memory thirdPartyTransfersForbiddenValues = new bool[](1);
        thirdPartyTransfersForbiddenValues[0] = false;
        strategyManager.addStrategiesToDepositWhitelist(strats, thirdPartyTransfersForbiddenValues);
    }

    function _deployHelloWorldContracts(
        IDelegationManager delegationManager,
        IAVSDirectory avsDirectory,
        IStrategy strat,
        address helloWorldCommunityMultisig,
        address helloWorldPauser
    ) internal {
        IStrategy[1] memory deployedStrategyArray = [strat];
        uint numStrategies = deployedStrategyArray.length;

        helloWorldProxyAdmin = new ProxyAdmin();

        {
            address[] memory pausers = new address[](2);
            pausers[0] = helloWorldPauser;
            pausers[1] = helloWorldCommunityMultisig;
            helloWorldPauserReg = new PauserRegistry(pausers, helloWorldCommunityMultisig);
        }

        EmptyContract emptyContract = new EmptyContract();

        helloWorldServiceManager = HelloWorldServiceManager(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(helloWorldProxyAdmin),
                    ""
                )
            )
        );
        registryCoordinator = RegistryCoordinator(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(helloWorldProxyAdmin),
                    ""
                )
            )
        );
        stakeRegistry = IStakeRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(helloWorldProxyAdmin),
                    ""
                )
            )
        );

        StakeRegistry stakeRegistryImplementation = new StakeRegistry(
            registryCoordinator,
            delegationManager
        );

        helloWorldProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(stakeRegistry))),
            address(stakeRegistryImplementation)
        );

        registryCoordinator = RegistryCoordinator(
            helloWorldServiceManager,
            IStakeRegistry(address(stakeRegistry)),
            IBLSApkRegistry(address("")),
            IIndexRegistry(address(""))
        );

        {
            helloWorldProxyAdmin.upgradeAndCall(
                TransparentUpgradeableProxy(payable(address(registryCoordinator))),
                address(registryCoordinatorImplementation),
                abi.encodeWithSelector(
                    RegistryCoordinator.initialize.selector,
                    helloWorldCommunityMultisig,
                    helloWorldCommunityMultisig,
                    helloWorldCommunityMultisig,
                    helloWorldPauserReg,
                    0,
                    [],
                    [],
                    []
                )
            );
        }

        HelloWorldServiceManager helloWorldServiceManagerImplementation = new HelloWorldServiceManager(
            avsDirectory,
            registryCoordinator,
            stakeRegistry
        );
        helloWorldProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(helloWorldServiceManager))),
            address(helloWorldServiceManagerImplementation)
        );

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(deployed_addresses, "erc20Mock", address(erc20Mock));
        vm.serializeAddress(deployed_addresses, "erc20MockStrategy", address(erc20MockStrategy));
        vm.serializeAddress(deployed_addresses, "HelloWorldServiceManager", address(helloWorldServiceManager));
        vm.serializeAddress(deployed_addresses, "HelloWorldServiceManagerImplementation", address(helloWorldServiceManagerImplementation));
        vm.serializeAddress(deployed_addresses, "registryCoordinator", address(registryCoordinator));
        vm.serializeAddress(deployed_addresses, "registryCoordinatorImplementation", address(registryCoordinatorImplementation));

        string memory finalJson = vm.serializeString("parent_object", deployed_addresses, "deployed_addresses_output");
        writeOutput(finalJson, "hello_world_avs_deployment_output");
    }
}