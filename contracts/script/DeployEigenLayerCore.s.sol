// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "@openzeppelin/contracts/proxy/beacon/UpgradeableBeacon.sol";

import "eigenlayer-contracts/src/contracts/interfaces/IETHPOSDeposit.sol";

import "eigenlayer-contracts/src/contracts/core/StrategyManager.sol";
import "eigenlayer-contracts/src/contracts/core/Slasher.sol";
import "eigenlayer-contracts/src/contracts/core/DelegationManager.sol";
import "eigenlayer-contracts/src/contracts/core/AVSDirectory.sol";
import "eigenlayer-contracts/src/contracts/core/RewardsCoordinator.sol";

import "eigenlayer-contracts/src/contracts/strategies/StrategyBaseTVLLimits.sol";

import "eigenlayer-contracts/src/contracts/pods/EigenPod.sol";
import "eigenlayer-contracts/src/contracts/pods/EigenPodManager.sol";

import "eigenlayer-contracts/src/contracts/permissions/PauserRegistry.sol";

import "eigenlayer-contracts/src/test/mocks/EmptyContract.sol";
import "eigenlayer-contracts/src/test/mocks/ETHDepositMock.sol";

import "forge-std/Script.sol";
import "forge-std/Test.sol";

import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";

// # To load the variables in the .env file
// source .env

// # To deploy and verify our contract
// forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json
contract DeployEigenlayerCore is Script, Test {
    using stdJson for string;
    using UpgradeableProxyLib for address;

    struct StrategyConfig {
        uint256 maxDeposits;
        uint256 maxPerDeposit;
        address tokenAddress;
        string tokenSymbol;
    }

    string internal deployConfigPath;

    // EigenLayer Contracts
    address internal eigenLayerProxyAdmin;
    PauserRegistry internal eigenLayerPauserReg;
    Slasher internal slasher;
    DelegationManager internal delegation;
    StrategyManager internal strategyManager;
    RewardsCoordinator internal rewardsCoordinator;
    AVSDirectory internal avsDirectory;
    EigenPodManager internal eigenPodManager;
    UpgradeableBeacon internal eigenPodBeacon;

    address internal executorMultisig;
    address internal operationsMultisig;
    address internal pauserMultisig;

    address[] internal pausers;

    // the ETH2 deposit contract -- if not on mainnet, we deploy a mock as stand-in
    IETHPOSDeposit internal ethPOSDeposit;

    // strategies deployed
    StrategyBaseTVLLimits[] internal deployedStrategyArray;
    StrategyConfig[] internal strategyConfigs;

    // IMMUTABLES TO SET
    uint64 internal GOERLI_GENESIS_TIME = 1_616_508_000;

    /// StrategyManager
    uint256 internal STRATEGY_MANAGER_INIT_PAUSED_STATUS;
    uint32 internal STRATEGY_MANAGER_INIT_WITHDRAWAL_DELAY_BLOCKS;

    /// DelegationManager
    uint256 internal DELEGATION_INIT_PAUSED_STATUS;
    uint256 internal DELEGATION_WITHDRAWAL_DELAY_BLOCKS;
    IStrategy[] internal strategies;
    uint256[] internal withdrawalDelayBlocks;

    /// Slasher
    uint256 internal SLASHER_INIT_PAUSED_STATUS;

    /// EigenPod Manager
    uint256 internal EIGENPOD_MANAGER_INIT_PAUSED_STATUS;

    // RewardsCoordinator
    uint256 internal REWARDS_COORDINATOR_INIT_PAUSED_STATUS;
    uint32 internal REWARDS_COORDINATOR_MAX_REWARDS_DURATION;
    uint32 internal REWARDS_COORDINATOR_MAX_RETROACTIVE_LENGTH;
    uint32 internal REWARDS_COORDINATOR_MAX_FUTURE_LENGTH;
    uint32 internal REWARDS_COORDINATOR_GENESIS_REWARDS_TIMESTAMP;
    address internal REWARDS_COORDINATOR_UPDATER;
    uint32 internal REWARDS_COORDINATOR_ACTIVATION_DELAY;
    uint32 internal REWARDS_COORDINATOR_CALCULATION_INTERVAL_SECONDS;
    uint32 internal REWARDS_COORDINATOR_GLOBAL_OPERATOR_COMMISSION_BIPS;

    function setUp() public virtual {
        _setUpCoreContractsConfig();
    }

    function run() external {
        vm.startBroadcast();
        _deployCoreContracts();
        vm.stopBroadcast();
    }

    function _setUpCoreContractsConfig() internal {
        /// TODO: Better parameterization of this
        string memory configFileName = "M2_deploy_from_scratch.anvil.config.json";
        // READ JSON CONFIG DATA
        deployConfigPath = string(
            bytes(
                string.concat(
                    "lib/eigenlayer-middleware/lib/eigenlayer-contracts/script/configs/devnet/",
                    configFileName
                )
            )
        );
        string memory configData = vm.readFile(deployConfigPath);
        // bytes memory parsedData = vm.parseJson(config_data);

        STRATEGY_MANAGER_INIT_PAUSED_STATUS =
            configData.readUint(".strategyManager.init_paused_status");
        SLASHER_INIT_PAUSED_STATUS = configData.readUint(".slasher.init_paused_status");
        DELEGATION_INIT_PAUSED_STATUS = configData.readUint(".delegation.init_paused_status");
        DELEGATION_WITHDRAWAL_DELAY_BLOCKS =
            configData.readUint(".delegation.init_withdrawal_delay_blocks");
        EIGENPOD_MANAGER_INIT_PAUSED_STATUS =
            configData.readUint(".eigenPodManager.init_paused_status");
        REWARDS_COORDINATOR_INIT_PAUSED_STATUS =
            configData.readUint(".rewardsCoordinator.init_paused_status");
        REWARDS_COORDINATOR_CALCULATION_INTERVAL_SECONDS =
            uint32(configData.readUint(".rewardsCoordinator.CALCULATION_INTERVAL_SECONDS"));
        REWARDS_COORDINATOR_MAX_REWARDS_DURATION =
            uint32(configData.readUint(".rewardsCoordinator.MAX_REWARDS_DURATION"));
        REWARDS_COORDINATOR_MAX_RETROACTIVE_LENGTH =
            uint32(configData.readUint(".rewardsCoordinator.MAX_RETROACTIVE_LENGTH"));
        REWARDS_COORDINATOR_MAX_FUTURE_LENGTH =
            uint32(configData.readUint(".rewardsCoordinator.MAX_FUTURE_LENGTH"));
        REWARDS_COORDINATOR_GENESIS_REWARDS_TIMESTAMP =
            uint32(configData.readUint(".rewardsCoordinator.GENESIS_REWARDS_TIMESTAMP"));
        REWARDS_COORDINATOR_UPDATER =
            configData.readAddress(".rewardsCoordinator.rewards_updater_address");
        REWARDS_COORDINATOR_ACTIVATION_DELAY =
            uint32(configData.readUint(".rewardsCoordinator.activation_delay"));
        REWARDS_COORDINATOR_CALCULATION_INTERVAL_SECONDS =
            uint32(configData.readUint(".rewardsCoordinator.calculation_interval_seconds"));
        REWARDS_COORDINATOR_GLOBAL_OPERATOR_COMMISSION_BIPS =
            uint32(configData.readUint(".rewardsCoordinator.global_operator_commission_bips"));

        STRATEGY_MANAGER_INIT_WITHDRAWAL_DELAY_BLOCKS =
            uint32(configData.readUint(".strategyManager.init_withdrawal_delay_blocks"));

        executorMultisig = configData.readAddress(".multisig_addresses.executorMultisig");
        operationsMultisig = configData.readAddress(".multisig_addresses.operationsMultisig");
        pauserMultisig = configData.readAddress(".multisig_addresses.pauserMultisig");

        pausers = new address[](3);
        (pausers[0], pausers[1], pausers[2]) =
            (executorMultisig, operationsMultisig, pauserMultisig);

        // load token list
        bytes memory strategyConfigsRaw = configData.parseRaw(".strategies");
        // tokens to deploy strategies for
        StrategyConfig[] memory strategyConfigsTemp;
        strategyConfigsTemp = abi.decode(strategyConfigsRaw, (StrategyConfig[]));
        for (uint256 i; i < strategyConfigsTemp.length; i++) {
            strategyConfigs.push(strategyConfigsTemp[i]);
        }

        // if on mainnet, use the ETH2 deposit contract address
        if (block.chainid == 1) {
            ethPOSDeposit = IETHPOSDeposit(0x00000000219ab540356cBB839Cbe05303d7705Fa);
            // if not on mainnet, deploy a mock
        } else {
            ethPOSDeposit = IETHPOSDeposit(configData.readAddress(".ethPOSDepositAddress"));
        }

        require(
            executorMultisig != address(0), "executorMultisig address not configured correctly!"
        );
        require(
            operationsMultisig != address(0), "operationsMultisig address not configured correctly!"
        );
    }

    function _deployCoreContracts() internal {
        eigenLayerProxyAdmin = UpgradeableProxyLib.deployProxyAdmin();

        eigenLayerPauserReg = new PauserRegistry(pausers, executorMultisig);

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        delegation = DelegationManager(eigenLayerProxyAdmin.setUpEmptyProxy());
        strategyManager = StrategyManager(eigenLayerProxyAdmin.setUpEmptyProxy());
        avsDirectory = AVSDirectory(eigenLayerProxyAdmin.setUpEmptyProxy());
        slasher = Slasher(eigenLayerProxyAdmin.setUpEmptyProxy());
        eigenPodManager = EigenPodManager(eigenLayerProxyAdmin.setUpEmptyProxy());
        rewardsCoordinator = RewardsCoordinator(eigenLayerProxyAdmin.setUpEmptyProxy());

        EigenPod eigenPodImplementation =
            new EigenPod(ethPOSDeposit, eigenPodManager, GOERLI_GENESIS_TIME);
        /// TODO: what is the holesky genesis time?

        eigenPodBeacon = new UpgradeableBeacon(address(eigenPodImplementation));

        // Second, deploy the *implementation* contracts, using the *proxy contracts* as inputs
        address delegationImpl =
            address(new DelegationManager(strategyManager, slasher, eigenPodManager));
        address strategyManagerImpl =
            address(new StrategyManager(delegation, eigenPodManager, slasher));
        address avsDirectoryImpl = address(new AVSDirectory(delegation));
        address slasherImpl = address(new Slasher(strategyManager, delegation));
        address eigenPodManagerImpl = address(
            new EigenPodManager(ethPOSDeposit, eigenPodBeacon, strategyManager, slasher, delegation)
        );
        address rewardsCoordinatorImpl = address(
            new RewardsCoordinator(
                delegation,
                strategyManager,
                REWARDS_COORDINATOR_CALCULATION_INTERVAL_SECONDS,
                REWARDS_COORDINATOR_MAX_REWARDS_DURATION,
                REWARDS_COORDINATOR_MAX_RETROACTIVE_LENGTH,
                REWARDS_COORDINATOR_MAX_FUTURE_LENGTH,
                REWARDS_COORDINATOR_GENESIS_REWARDS_TIMESTAMP
            )
        );

        // Third, upgrade the proxy contracts to use the correct implementation contracts and initialize them.

        bytes memory upgradeDelegationData = abi.encodeCall(
            DelegationManager.initialize,
            (
                executorMultisig,
                eigenLayerPauserReg,
                DELEGATION_INIT_PAUSED_STATUS,
                DELEGATION_WITHDRAWAL_DELAY_BLOCKS,
                strategies,
                withdrawalDelayBlocks
            )
        );
        address(delegation).upgradeAndCall(delegationImpl, upgradeDelegationData);

        bytes memory upgradeStrategyManagerData = abi.encodeCall(
            StrategyManager.initialize,
            (
                executorMultisig,
                operationsMultisig,
                eigenLayerPauserReg,
                STRATEGY_MANAGER_INIT_PAUSED_STATUS
            )
        );
        address(strategyManager).upgradeAndCall(strategyManagerImpl, upgradeStrategyManagerData);

        bytes memory upgradeSlasherData = abi.encodeCall(
            Slasher.initialize, (executorMultisig, eigenLayerPauserReg, SLASHER_INIT_PAUSED_STATUS)
        );
        address(slasher).upgradeAndCall(slasherImpl, upgradeSlasherData);

        bytes memory upgradeAVSDirectoryData =
            abi.encodeCall(AVSDirectory.initialize, (executorMultisig, eigenLayerPauserReg, 0));
        address(avsDirectory).upgradeAndCall(avsDirectoryImpl, upgradeAVSDirectoryData);

        bytes memory upgradeEigenPodManagerData = abi.encodeCall(
            EigenPodManager.initialize,
            (executorMultisig, eigenLayerPauserReg, EIGENPOD_MANAGER_INIT_PAUSED_STATUS)
        );
        address(eigenPodManager).upgradeAndCall(eigenPodManagerImpl, upgradeEigenPodManagerData);

        bytes memory upgradeRewardsCoordinatorData = abi.encodeCall(
            RewardsCoordinator.initialize,
            (
                executorMultisig,
                eigenLayerPauserReg,
                REWARDS_COORDINATOR_INIT_PAUSED_STATUS,
                REWARDS_COORDINATOR_UPDATER,
                REWARDS_COORDINATOR_ACTIVATION_DELAY,
                /// TODO: casting issue before
                uint16(REWARDS_COORDINATOR_GLOBAL_OPERATOR_COMMISSION_BIPS)
            )
        );
        address(rewardsCoordinator).upgradeAndCall(
            address(rewardsCoordinatorImpl), upgradeRewardsCoordinatorData
        );

        // deploy StrategyBaseTVLLimits contract implementation
        address baseStrategyImpl = address(new StrategyBaseTVLLimits(strategyManager));
        // create upgradeable proxies that each point to the implementation and initialize them

        bytes memory strategyInitCall;
        for (uint256 i = 0; i < strategyConfigs.length; ++i) {
            strategyInitCall = abi.encodeCall(
                StrategyBaseTVLLimits.initialize,
                (
                    strategyConfigs[i].maxPerDeposit,
                    strategyConfigs[i].maxDeposits,
                    IERC20(strategyConfigs[i].tokenAddress),
                    eigenLayerPauserReg
                )
            );

            deployedStrategyArray.push(
                StrategyBaseTVLLimits(
                    address(
                        new TransparentUpgradeableProxy(
                            baseStrategyImpl, eigenLayerProxyAdmin, strategyInitCall
                        )
                    )
                )
            );
        }

        ProxyAdmin(eigenLayerProxyAdmin).transferOwnership(executorMultisig);
        eigenPodBeacon.transferOwnership(executorMultisig);
    }
}
