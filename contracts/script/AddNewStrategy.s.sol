// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IStrategyManager, IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {ISlasher} from "@eigenlayer/contracts/interfaces/ISlasher.sol";
import {StrategyBaseTVLLimits} from "@eigenlayer/contracts/strategies/StrategyBaseTVLLimits.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {Quorum, StrategyParams} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import "@eigenlayer-middleware/src/OperatorStateRetriever.sol";
import {HelloWorldServiceManager, IServiceManager} from "../src/HelloWorldServiceManager.sol";
import "../src/ERC20Mock.sol";
import {Utils} from "./utils/Utils.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/Test.sol";
import "forge-std/console.sol";

contract AddStrategyScript is Script, Utils {
    // Replace the placeholders with actual addresses or add them as inputs to the script
    address eigenLayerProxyAdminAddr;
    address eigenLayerPauserRegAddr;
    address baseStrategyImplementationAddr;
    address strategyManagerAddr;
    address delegationManagerAddr;
    address avsDirectoryAddr;
    address helloWorldServiceManagerProxyAddr;
    address stakeRegistryProxyAddr;

    function run() external {
        ERC20Mock erc20Mock = new ERC20Mock();
        StrategyBaseTVLLimits erc20MockStrategy = _deployStrategy(erc20Mock);
        _whitelistStrategy(erc20MockStrategy);
        _updateHelloWorldAVS(erc20MockStrategy);
    }

    function _deployStrategy(ERC20Mock erc20Mock) internal returns (StrategyBaseTVLLimits) {
        ProxyAdmin eigenLayerProxyAdmin = ProxyAdmin(eigenLayerProxyAdminAddr);
        PauserRegistry eigenLayerPauserReg = PauserRegistry(eigenLayerPauserRegAddr);
        StrategyBaseTVLLimits baseStrategyImplementation = StrategyBaseTVLLimits(baseStrategyImplementationAddr);

        return StrategyBaseTVLLimits(
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
    }

    function _whitelistStrategy(StrategyBaseTVLLimits erc20MockStrategy) internal {
        IStrategyManager strategyManager = IStrategyManager(strategyManagerAddr);

        IStrategy[] memory strats = new IStrategy[](1);
        strats[0] = erc20MockStrategy;
        bool[] memory thirdPartyTransfersForbiddenValues = new bool[](1);
        thirdPartyTransfersForbiddenValues[0] = false;
        strategyManager.addStrategiesToDepositWhitelist(
            strats,
            thirdPartyTransfersForbiddenValues
        );
    }

    function _updateHelloWorldAVS(StrategyBaseTVLLimits erc20MockStrategy) internal {
        IDelegationManager delegationManager = IDelegationManager(delegationManagerAddr);
        IAVSDirectory avsDirectory = IAVSDirectory(avsDirectoryAddr);
        HelloWorldServiceManager helloWorldServiceManagerProxy = HelloWorldServiceManager(helloWorldServiceManagerProxyAddr);
        ECDSAStakeRegistry stakeRegistryProxy = ECDSAStakeRegistry(stakeRegistryProxyAddr);

        StrategyParams memory strategyParams = StrategyParams({
            strategy: erc20MockStrategy,
            multiplier: 10_000
        });

        StrategyParams[] memory strategies = new StrategyParams[](1);
        strategies[0] = strategyParams;

        Quorum memory quorum = Quorum({
            strategies: strategies
        });

        stakeRegistryProxy.initialize(
            address(helloWorldServiceManagerProxy),
            1,
            quorum
        );
    }
}
