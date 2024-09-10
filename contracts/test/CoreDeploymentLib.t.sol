// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";
import {console2} from "forge-std/console2.sol";
import {CoreDeploymentLib} from "../script/utils/CoreDeploymentLib.sol";
import {UpgradeableProxyLib} from "../script/utils/UpgradeableProxyLib.sol";

contract CoreDeploymentLibTest is Test {
    using CoreDeploymentLib for *;
    using UpgradeableProxyLib for address;

    address proxyAdmin;
    CoreDeploymentLib.DeploymentData deploymentData;
    CoreDeploymentLib.DeploymentConfigData configData;

    function setUp() public {
        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
    }

    /// won't test specific functionality/values. Testing behavior of the library
    function test_ReadConfig() public {
        CoreDeploymentLib.readDeploymentConfigValues("test/mockData/config/core/", 1337);
    }

    function test_ReadConfig_Reverts() public {
        vm.expectRevert();
        /// Incorrect path
        CoreDeploymentLib.readDeploymentConfigValues("test/mockData/deployments/core/", 1337);
    }

    function test_ReadDeployment() public {
        CoreDeploymentLib.readDeploymentJson("test/mockData/deployments/core/", 1337);
    }

    function test_ReadDeployment_Reverts() public {
        vm.expectRevert();
        /// Incorrect path
        CoreDeploymentLib.readDeploymentJson("test/mockData/config/core/", 1337);
    }

    function test_DeployContracts() public {
        CoreDeploymentLib.DeploymentData memory data = CoreDeploymentLib.deployContracts(proxyAdmin);

        assertTrue(data.delegationManager != address(0), "DelegationManager not deployed");
        assertTrue(data.avsDirectory != address(0), "AVSDirectory not deployed");
        assertTrue(data.strategyManager != address(0), "StrategyManager not deployed");
    }

    function test_WriteDeploymentJson() public {
        CoreDeploymentLib.DeploymentData memory data = CoreDeploymentLib.deployContracts(proxyAdmin);

        string memory scratchPath = "test/mockData/scratch/";
        CoreDeploymentLib.writeDeploymentJson(scratchPath, block.chainid, data);

        string memory fileName = string.concat(scratchPath, vm.toString(block.chainid), ".json");
        assertTrue(vm.exists(fileName), "Deployment file not created");
    }

    function test_ReadConfigFromM2DeploymentData() public {
        // Path to the M2 deployment data JSON file
        string memory m2DeploymentDataPath =
            "lib/eigenlayer-middleware/lib/eigenlayer-contracts/script/output/";

        /// TODO: this is going to revert for now
        vm.expectRevert();
        CoreDeploymentLib.readDeploymentJson(m2DeploymentDataPath, block.chainid);
    }
}
