### Guide: Adding a New Strategy to the Hello World AVS

This guide will help you add a new strategy to the Hello World AVS using Foundry scripts.

### Prerequisites

Before starting, ensure you have the following:
- A development environment set up with Foundry.
- Access to the EigenLayer contracts and the Hello World AVS contracts.
- A deployed instance of the Hello World AVS.
- The following addresses:
  - Proxy Admin Address
  - Pauser Registry Address
  - Strategy Base Implementation Address
  - Strategy Manager Address
  - Delegation Manager Address
  - AVS Directory Address
  - Hello World Service Manager Proxy Address
  - ECDSA Stake Registry Proxy Address

### Step 1: Setup Your Environment

Ensure your development environment is set up with Foundry and you have cloned the necessary repositories. Navigate to the directory where your Foundry project is located.

### Step 2: Prepare the Script

In your project directory, locate or create the script file (e.g., `AddNewStrategy.s.sol`) where you will write the script to deploy and configure the new strategy.

### Step 3: Script Outline

Your script should follow this general outline:

1. **Imports**: Ensure you have all the necessary imports.
2. **State Variables**: Define state variables for addresses to avoid passing them as arguments multiple times.
3. **Main Function**: Implement the `run` function which will be the entry point.
4. **Helper Functions**: Break down the logic into smaller helper functions to avoid stack too deep issues.

### Step 4: Define State Variables

Define the addresses you will use as state variables at the top of your contract to keep your code clean and manageable.

```solidity
address eigenLayerProxyAdminAddr = /* Proxy Admin Address */;
address eigenLayerPauserRegAddr = /* Pauser Registry Address */;
address baseStrategyImplementationAddr = /* Strategy Base Implementation Address */;
address strategyManagerAddr = /* Strategy Manager Address */;
address delegationManagerAddr = /* Delegation Manager Address */;
address avsDirectoryAddr = /* AVS Directory Address */;
address helloWorldServiceManagerProxyAddr = /* Hello World Service Manager Proxy Address */;
address stakeRegistryProxyAddr = /* ECDSA Stake Registry Proxy Address */;
```

### Step 5: Implement the `run` Function

This function will serve as the entry point to deploy the strategy and update the Hello World AVS.

```solidity
function run() external {
    ERC20Mock erc20Mock = new ERC20Mock();
    StrategyBaseTVLLimits erc20MockStrategy = _deployStrategy(erc20Mock);
    _whitelistStrategy(erc20MockStrategy);
    _updateHelloWorldAVS(erc20MockStrategy);
}
```

### Step 6: Helper Functions

Implement the helper functions to handle the deployment, whitelisting, and updating the AVS.

#### Deploy the Strategy

```solidity
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
```

#### Whitelist the Strategy

```solidity
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
```

#### Update the Hello World AVS

```solidity
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
```

### Step 7: Run the Script

To deploy and verify the contract, run the script using the following Foundry command:

```bash
forge script script/AddNewStrategy.s.sol:AddStrategyScript --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -v
```

### Conclusion

This guide provides a step-by-step process to add a new strategy to the Hello World AVS using Foundry scripts. By following these steps, you should be able to deploy and integrate new strategies seamlessly. If you encounter any issues, feel free to reach out for support.