// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";
import {console2} from "forge-std/console2.sol";
import {
    CoreDeployLib, CoreDeploymentParsingLib
} from "../script/utils/CoreDeploymentParsingLib.sol";
import {UpgradeableProxyLib} from "../script/utils/UpgradeableProxyLib.sol";

contract CoreDeploymentParsingLibTest is Test {
    using UpgradeableProxyLib for address;

    address proxyAdmin;
    CoreDeployLib.DeploymentData deploymentData;
    CoreDeployLib.DeploymentConfigData configData;

    function setUp() public {
        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
    }

    /// won't test specific functionality/values. Testing behavior of the library
    function test_ReadConfig() public view {
        CoreDeploymentParsingLib.readDeploymentConfigValues("test/mockData/config/core/", 1337);
    }

    /// forge-config: default.allow_internal_expect_revert = true
    function test_ReadConfig_Reverts() public {
        vm.expectRevert();
        /// Incorrect path
        CoreDeploymentParsingLib.readDeploymentConfigValues("test/mockData/deployments/core/", 1337);
    }

    function test_ReadDeployment() public view {
        CoreDeploymentParsingLib.readDeploymentJson("test/mockData/deployments/core/", 1337);
    }

    /// forge-config: default.allow_internal_expect_revert = true
    function test_ReadDeployment_Reverts() public {
        vm.expectRevert();
        /// Incorrect path
        CoreDeploymentParsingLib.readDeploymentJson("test/mockData/config/core/", 1337);
    }

    function test_DeployContracts() public {
        configData =
            CoreDeploymentParsingLib.readDeploymentConfigValues("test/mockData/config/core/", 1337);
        deploymentData = CoreDeployLib.deployContracts(proxyAdmin, configData);

        assertTrue(deploymentData.delegationManager != address(0), "DelegationManager not deployed");
        assertTrue(deploymentData.avsDirectory != address(0), "AVSDirectory not deployed");
        assertTrue(deploymentData.strategyManager != address(0), "StrategyManager not deployed");
    }

    function test_WriteDeploymentJson() public {
        configData =
            CoreDeploymentParsingLib.readDeploymentConfigValues("test/mockData/config/core/", 1337);
        deploymentData = CoreDeployLib.deployContracts(proxyAdmin, configData);

        string memory scratchPath = "test/mockData/scratch/test_WriteDeploymentJson/";
        CoreDeploymentParsingLib.writeDeploymentJson(scratchPath, block.chainid, deploymentData);

        string memory fileName = string.concat(scratchPath, vm.toString(block.chainid), ".json");
        assertTrue(vm.exists(fileName), "Deployment file not created");

        vm.removeFile(fileName);
    }

    function test_WriteAndReadDeploymentJson() public {
        configData =
            CoreDeploymentParsingLib.readDeploymentConfigValues("test/mockData/config/core/", 1337);
        deploymentData = CoreDeployLib.deployContracts(proxyAdmin, configData);

        string memory scratchPath = "test/mockData/scratch/test_WriteAndReadDeploymentJson/";

        CoreDeploymentParsingLib.writeDeploymentJson(scratchPath, block.chainid, deploymentData);

        string memory fileName = string.concat(vm.toString(block.chainid), ".json");

        CoreDeploymentParsingLib.readDeploymentJson(scratchPath, fileName);

        vm.removeFile(string.concat(scratchPath, fileName));
    }

    function test_ReadConfigFromM2DeploymentData() public {
        /// TODO: Deployment json is missing the strategy factory
        vm.skip(true);
        // Path to the M2 deployment data JSON file
        string memory m2DeploymentDataPath =
            "lib/eigenlayer-middleware/lib/eigenlayer-contracts/script/output/devnet/";
        string memory m2DeploymentFilename = "M2_from_scratch_deployment_data.json";

        CoreDeploymentParsingLib.readDeploymentJson(m2DeploymentDataPath, m2DeploymentFilename);
    }
}
