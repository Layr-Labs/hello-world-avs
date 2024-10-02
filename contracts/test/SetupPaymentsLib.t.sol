// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../contracts/script/utils/SetupPaymentsLib.sol";
import "../contracts/script/utils/CoreDeploymentLib.sol";
import "../contracts/script/utils/HelloWorldDeploymentLib.sol";
import "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import "@eigenlayer/contracts/interfaces/IStrategy.sol";
import "../contracts/script/DeployEigenLayerCore.s.sol";
import "../contracts/script/DeployHelloWorld.s.sol";

contract SetupPaymentsLibTest is Test {
    using SetupPaymentsLib for *;

    IRewardsCoordinator public rewardsCoordinator;
    IStrategy public strategy;
    address deployer;
    CoreDeploymentLib.DeploymentData coreDeployment;
    HelloWorldDeploymentLib.DeploymentData helloWorldDeployment;

    uint256 constant NUM_PAYMENTS = 8;
    uint256 constant NUM_TOKEN_EARNINGS = 1;
    uint256 constant TOKEN_EARNINGS = 100;

    function setUp() public {
        deployer = vm.addr(1);
        vm.startPrank(deployer);

        // Deploy EigenLayer Core
        DeployEigenLayerCore deployEigenLayerScript = new DeployEigenLayerCore();
        deployEigenLayerScript.run();

        // Deploy HelloWorld contracts
        DeployHelloWorld deployHelloWorldScript = new DeployHelloWorld();
        deployHelloWorldScript.run();

        // Read deployment data
        coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);
        helloWorldDeployment = HelloWorldDeploymentLib.readDeploymentJson("deployments/hello-world/", block.chainid);

        rewardsCoordinator = IRewardsCoordinator(coreDeployment.rewardsCoordinator);
        strategy = IStrategy(helloWorldDeployment.strategy);

        vm.stopPrank();
    }

    function testCreatePaymentSubmissions() public {
        uint256 numPayments = 5;
        uint256 amountPerPayment = 100;
        uint32 duration = 7 days;

        vm.startPrank(deployer);
        SetupPaymentsLib.createPaymentSubmissions(
            rewardsCoordinator,
            address(strategy),
            numPayments,
            amountPerPayment,
            duration
        );
        vm.stopPrank();

    }

    function testProcessClaim() public {
        string memory filePath = "test_payments.json";
        uint256 indexToProve = 0;
        address recipient = address(0x1111111111111111111111111111111111111111);
        IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf;

        // Create a test JSON file
        string memory jsonContent = '{"leaves":["0x1234"], "tokenLeaves":["0x5678"]}';
        vm.writeFile(filePath, jsonContent);

        vm.startPrank(deployer);
        SetupPaymentsLib.processClaim(
            rewardsCoordinator,
            filePath,
            indexToProve,
            recipient,
            earnerLeaf,
            NUM_TOKEN_EARNINGS,
            address(strategy)
        );
        vm.stopPrank();

    }

    function testSubmitPaymentRoot() public {
        address[] memory earners = new address[](2);
        earners[0] = address(0x1111111111111111111111111111111111111111);
        earners[1] = address(0x2222222222222222222222222222222222222222);

        vm.startPrank(deployer);
        SetupPaymentsLib.submitPaymentRoot(rewardsCoordinator, earners);
        vm.stopPrank();

        // Add assertions to verify the root submission
        // You may need to call view functions on the rewardsCoordinator to check the state
    }

    function testCreatePaymentRoot() public {
        address[] memory earners = new address[](2);
        earners[0] = address(0x1111111111111111111111111111111111111111);
        earners[1] = address(0x2222222222222222222222222222222222222222);

        vm.startPrank(deployer);
        bytes32 root = SetupPaymentsLib.createPaymentRoot(
            rewardsCoordinator,
            earners,
            2,
            1,
            100,
            address(strategy),
            vm
        );
        vm.stopPrank();

        assertNotEq(root, bytes32(0), "Root should not be zero");
    }


    function testWriteLeavesToJson() public {
        bytes32[] memory leaves = new bytes32[](2);
        leaves[0] = bytes32(uint256(1));
        leaves[1] = bytes32(uint256(2));

        bytes32[] memory tokenLeaves = new bytes32[](2);
        tokenLeaves[0] = bytes32(uint256(3));
        tokenLeaves[1] = bytes32(uint256(4));

        SetupPaymentsLib.writeLeavesToJson(leaves, tokenLeaves, vm);

        assertTrue(vm.exists("payments.json"), "JSON file should be created");
    }

    function testParseLeavesFromJson() public {
        string memory filePath = "test_parse_payments.json";
        string memory jsonContent = '{"leaves":["0x1234"], "tokenLeaves":["0x5678"]}';
        vm.writeFile(filePath, jsonContent);

        SetupPaymentsLib.PaymentLeaves memory paymentLeaves = SetupPaymentsLib.parseLeavesFromJson(filePath, vm);

        assertEq(paymentLeaves.leaves.length, 1, "Incorrect number of leaves");
        assertEq(paymentLeaves.tokenLeaves.length, 1, "Incorrect number of token leaves");
    }
}