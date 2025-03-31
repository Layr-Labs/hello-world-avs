// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../script/utils/SetupDistributionsLib.sol";
import "../script/utils/CoreDeploymentParsingLib.sol";
import "../script/utils/HelloWorldDeploymentLib.sol";
import "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import "../src/IHelloWorldServiceManager.sol";
import "@eigenlayer/contracts/interfaces/IStrategy.sol";
import "@eigenlayer/contracts/libraries/Merkle.sol";
import "../script/DeployEigenLayerCore.s.sol";
import "../script/HelloWorldDeployer.s.sol";
import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";
import {HelloWorldTaskManagerSetup} from "test/HelloWorldServiceManager.t.sol";
import {ECDSAServiceManagerBase} from
    "@eigenlayer-middleware/src/unaudited/ECDSAServiceManagerBase.sol";
import {
    IECDSAStakeRegistryTypes,
    IStrategy
} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistry.sol";
import "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";

contract TestConstants {
    uint256 constant NUM_PAYMENTS = 8;
    uint256 constant NUM_TOKEN_EARNINGS = 1;
    uint256 constant TOKEN_EARNINGS = 100;

    address RECIPIENT = address(1);
    address EARNER = address(2);
    uint256 INDEX_TO_PROVE = 0;
    uint256 NUM_EARNERS = 4;
}

contract SetupDistributionsLibTest is Test, TestConstants, HelloWorldTaskManagerSetup {
    using SetupDistributionsLib for *;

    Vm cheats = Vm(VM_ADDRESS);

    IRewardsCoordinator public rewardsCoordinator;
    IHelloWorldServiceManager public helloWorldServiceManager;
    IStrategy public strategy;

    address rewardsInitiator = address(1);
    address rewardsOwner = address(2);

    function setUp() public virtual override {
        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        coreConfigData =
            CoreDeploymentParsingLib.readDeploymentConfigValues("test/mockData/config/core/", 1337);
        coreDeployment = CoreDeployLib.deployContracts(proxyAdmin, coreConfigData);

        vm.prank(coreConfigData.strategyManager.initialOwner);
        StrategyManager(coreDeployment.strategyManager).setStrategyWhitelister(
            coreDeployment.strategyFactory
        );

        mockToken = new ERC20Mock();

        strategy = addStrategy(address(mockToken)); // Similar function to HW_SM test using strategy factory
        quorum.strategies.push(
            IECDSAStakeRegistryTypes.StrategyParams({strategy: strategy, multiplier: 10_000})
        );

        helloWorldDeployment = HelloWorldDeploymentLib.deployContracts(
            proxyAdmin, coreDeployment, quorum, rewardsInitiator, rewardsOwner
        );
        labelContracts(coreDeployment, helloWorldDeployment);

        cheats.prank(rewardsOwner);
        ECDSAServiceManagerBase(helloWorldDeployment.helloWorldServiceManager).setRewardsInitiator(
            rewardsInitiator
        );

        rewardsCoordinator = IRewardsCoordinator(coreDeployment.rewardsCoordinator);

        mockToken.mint(address(this), 100_000);
        mockToken.mint(address(rewardsCoordinator), 100_000);
        mockToken.mint(rewardsInitiator, 100_000);
    }

    function testSubmitRoot() public {
        address[] memory earners = new address[](NUM_EARNERS);
        for (uint256 i = 0; i < earners.length; i++) {
            earners[i] = address(1);
        }
        uint32 endTimestamp = rewardsCoordinator.currRewardsCalculationEndTimestamp() + 1 weeks;
        cheats.warp(endTimestamp + 1);

        bytes32[] memory tokenLeaves = SetupDistributionsLib.createTokenLeaves(
            rewardsCoordinator, NUM_TOKEN_EARNINGS, TOKEN_EARNINGS, address(strategy)
        );
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves =
            SetupDistributionsLib.createEarnerLeaves(earners, tokenLeaves);

        string memory filePath = "testSubmitRoot.json";

        cheats.startPrank(rewardsCoordinator.rewardsUpdater());
        SetupDistributionsLib.submitRoot(
            rewardsCoordinator, tokenLeaves, earnerLeaves, endTimestamp, NUM_EARNERS, 1, filePath
        );
        cheats.stopPrank();
        vm.removeFile(filePath);
    }

    function testWriteLeavesToJson() public {
        bytes32[] memory leaves = new bytes32[](2);
        leaves[0] = bytes32(uint256(1));
        leaves[1] = bytes32(uint256(2));

        bytes32[] memory tokenLeaves = new bytes32[](2);
        tokenLeaves[0] = bytes32(uint256(3));
        tokenLeaves[1] = bytes32(uint256(4));

        string memory filePath = "testWriteLeavesToJson.json";

        SetupDistributionsLib.writeLeavesToJson(leaves, tokenLeaves, filePath);

        assertTrue(vm.exists(filePath), "JSON file should be created");
        vm.removeFile(filePath);
    }

    function testParseLeavesFromJson() public {
        string memory filePath = "test_parse_payments.json";
        string memory jsonContent = '{"leaves":["0x1234"], "tokenLeaves":["0x5678"]}';
        vm.writeFile(filePath, jsonContent);

        SetupDistributionsLib.PaymentLeaves memory paymentLeaves =
            SetupDistributionsLib.parseLeavesFromJson(filePath);

        assertEq(paymentLeaves.leaves.length, 1, "Incorrect number of leaves");
        assertEq(paymentLeaves.tokenLeaves.length, 1, "Incorrect number of token leaves");

        vm.removeFile(filePath);
    }

    function testGenerateMerkleProof() public view {
        SetupDistributionsLib.PaymentLeaves memory paymentLeaves =
            SetupDistributionsLib.parseLeavesFromJson("test/mockData/scratch/payments_test.json");

        bytes32[] memory leaves = paymentLeaves.leaves;
        uint256 indexToProve = 0;

        bytes32[] memory proof = new bytes32[](2);
        proof[0] = leaves[1];
        proof[1] = keccak256(abi.encodePacked(leaves[2], leaves[3]));

        bytes memory proofBytesConstructed = abi.encodePacked(proof);
        bytes memory proofBytesCalculated =
            SetupDistributionsLib.generateMerkleProof(leaves, indexToProve);

        require(
            keccak256(proofBytesConstructed) == keccak256(proofBytesCalculated),
            "Proofs do not match"
        );

        bytes32 root = SetupDistributionsLib.merkleizeKeccak(leaves);

        require(
            Merkle.verifyInclusionKeccak(
                proofBytesCalculated, root, leaves[indexToProve], indexToProve
            )
        );
    }

    function testProcessClaim() public {
        emit log_named_address("token address", address(mockToken));
        string memory filePath = "testProcessClaim.json";

        address[] memory earners = new address[](NUM_EARNERS);
        for (uint256 i = 0; i < earners.length; i++) {
            earners[i] = address(1);
        }
        uint32 endTimestamp = rewardsCoordinator.currRewardsCalculationEndTimestamp() + 1 weeks;
        cheats.warp(endTimestamp + 1);

        bytes32[] memory tokenLeaves = SetupDistributionsLib.createTokenLeaves(
            rewardsCoordinator, NUM_TOKEN_EARNINGS, TOKEN_EARNINGS, address(strategy)
        );
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves =
            SetupDistributionsLib.createEarnerLeaves(earners, tokenLeaves);

        cheats.startPrank(rewardsCoordinator.rewardsUpdater());
        SetupDistributionsLib.submitRoot(
            rewardsCoordinator, tokenLeaves, earnerLeaves, endTimestamp, NUM_EARNERS, 1, filePath
        );
        cheats.stopPrank();

        cheats.warp(block.timestamp + 2 weeks);

        cheats.startPrank(earnerLeaves[INDEX_TO_PROVE].earner, earnerLeaves[INDEX_TO_PROVE].earner);
        SetupDistributionsLib.processClaim(
            rewardsCoordinator,
            filePath,
            INDEX_TO_PROVE,
            RECIPIENT,
            earnerLeaves[INDEX_TO_PROVE],
            NUM_TOKEN_EARNINGS,
            address(strategy),
            uint32(TOKEN_EARNINGS)
        );

        cheats.stopPrank();

        vm.removeFile(filePath);
    }

    function testCreateAVSRewardsSubmissions() public {
        uint256 numPayments = 5;
        uint256 amountPerPayment = 100;
        uint32 duration = rewardsCoordinator.MAX_REWARDS_DURATION();
        uint32 genesisTimestamp = rewardsCoordinator.GENESIS_REWARDS_TIMESTAMP();
        uint32 startTimestamp = genesisTimestamp + 10 days;
        cheats.warp(startTimestamp + 1);

        cheats.prank(rewardsInitiator);
        mockToken.increaseAllowance(
            helloWorldDeployment.helloWorldServiceManager, amountPerPayment * numPayments
        );

        cheats.startPrank(rewardsInitiator);
        SetupDistributionsLib.createAVSRewardsSubmissions(
            address(helloWorldDeployment.helloWorldServiceManager),
            address(strategy),
            numPayments,
            amountPerPayment,
            duration,
            startTimestamp
        );
    }
}
