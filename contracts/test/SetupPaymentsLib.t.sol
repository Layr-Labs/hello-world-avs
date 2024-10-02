// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../contracts/script/utils/SetupPaymentsLib.sol";
import "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import "@eigenlayer/contracts/interfaces/IStrategy.sol";

contract MockRewardsCoordinator is IRewardsCoordinator {
    function createAVSRewardsSubmission(RewardsSubmission[] memory submissions) external {}
    function processClaim(RewardsMerkleClaim memory claim, address recipient) external {}
    function submitRoot(bytes32 root, uint32 rewardsCalculationEndTimestamp) external {}
    function currRewardsCalculationEndTimestamp() external view returns (uint32) { return uint32(block.timestamp); }
    function calculateEarnerLeafHash(EarnerTreeMerkleLeaf memory leaf) external pure returns (bytes32) {
        return keccak256(abi.encode(leaf));
    }
    function calculateTokenLeafHash(TokenTreeMerkleLeaf memory leaf) external pure returns (bytes32) {
        return keccak256(abi.encode(leaf));
    }
    function setRewardsUpdater(address newRewardsUpdater) external {}
}

contract MockStrategy is IStrategy {
    function underlyingToken() external view returns (address) {
        return address(0x1234567890123456789012345678901234567890);
    }
    // Implement other required functions...
}

contract SetupPaymentsLibTest is Test {
    using SetupPaymentsLib for *;

    MockRewardsCoordinator public rewardsCoordinator;
    MockStrategy public strategy;

    function setUp() public {
        rewardsCoordinator = new MockRewardsCoordinator();
        strategy = new MockStrategy();
    }

    function testCreatePaymentSubmissions() public {
        uint256 numPayments = 5;
        uint256 amountPerPayment = 100;
        uint32 duration = 7 days;

        vm.expectCall(
            address(rewardsCoordinator),
            abi.encodeWithSelector(IRewardsCoordinator.createAVSRewardsSubmission.selector)
        );

        SetupPaymentsLib.createPaymentSubmissions(
            rewardsCoordinator,
            address(strategy),
            numPayments,
            amountPerPayment,
            duration
        );
    }

    function testProcessClaim() public {
        string memory filePath = "test_payments.json";
        uint256 indexToProve = 0;
        address recipient = address(0x1111111111111111111111111111111111111111);
        IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf;
        uint256 NUM_TOKEN_EARNINGS = 1;

        // Create a mock JSON file
        string memory jsonContent = '{"leaves":["0x1234"], "tokenLeaves":["0x5678"]}';
        vm.writeFile(filePath, jsonContent);

        vm.expectCall(
            address(rewardsCoordinator),
            abi.encodeWithSelector(IRewardsCoordinator.processClaim.selector)
        );

        SetupPaymentsLib.processClaim(
            rewardsCoordinator,
            filePath,
            indexToProve,
            recipient,
            earnerLeaf,
            NUM_TOKEN_EARNINGS,
            address(strategy)
        );
    }

    function testSubmitPaymentRoot() public {
        address[] memory earners = new address[](2);
        earners[0] = address(0x1111111111111111111111111111111111111111);
        earners[1] = address(0x2222222222222222222222222222222222222222);

        vm.expectCall(
            address(rewardsCoordinator),
            abi.encodeWithSelector(IRewardsCoordinator.submitRoot.selector)
        );

        SetupPaymentsLib.submitPaymentRoot(rewardsCoordinator, earners);
    }

    function testCreatePaymentRoot() public {
        address[] memory earners = new address[](2);
        earners[0] = address(0x1111111111111111111111111111111111111111);
        earners[1] = address(0x2222222222222222222222222222222222222222);

        bytes32 root = SetupPaymentsLib.createPaymentRoot(
            rewardsCoordinator,
            earners,
            2,
            1,
            100,
            address(strategy),
            vm
        );

        assertNotEq(root, bytes32(0), "Root should not be zero");
    }

    function testCreateTokenRoot() public {
        bytes32[] memory tokenLeaves = new bytes32[](2);
        tokenLeaves[0] = bytes32(uint256(1));
        tokenLeaves[1] = bytes32(uint256(2));

        bytes32 root = SetupPaymentsLib.createTokenRoot(tokenLeaves);
        assertNotEq(root, bytes32(0), "Token root should not be zero");
    }

    function testCreateTokenLeaves() public {
        uint256 NUM_TOKEN_EARNINGS = 2;
        uint256 TOKEN_EARNINGS = 100;

        bytes32[] memory leaves = SetupPaymentsLib.createTokenLeaves(
            rewardsCoordinator,
            NUM_TOKEN_EARNINGS,
            TOKEN_EARNINGS,
            address(strategy)
        );

        assertEq(leaves.length, NUM_TOKEN_EARNINGS, "Incorrect number of token leaves");
    }

    function testDefaultTokenLeaf() public {
        uint256 TOKEN_EARNINGS = 100;

        IRewardsCoordinator.TokenTreeMerkleLeaf memory leaf = SetupPaymentsLib.defaultTokenLeaf(
            TOKEN_EARNINGS,
            address(strategy)
        );

        assertEq(leaf.token, strategy.underlyingToken(), "Incorrect token address");
        assertEq(leaf.cumulativeEarnings, TOKEN_EARNINGS, "Incorrect cumulative earnings");
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

    function testGenerateMerkleProof() public {
        bytes32[] memory leaves = new bytes32[](4);
        leaves[0] = bytes32(uint256(1));
        leaves[1] = bytes32(uint256(2));
        leaves[2] = bytes32(uint256(3));
        leaves[3] = bytes32(uint256(4));

        bytes memory proof = SetupPaymentsLib.generateMerkleProof(leaves, 2);
        assertGt(proof.length, 0, "Proof should not be empty");
    }

    function testMerkleizeKeccak() public {
        bytes32[] memory leaves = new bytes32[](4);
        leaves[0] = bytes32(uint256(1));
        leaves[1] = bytes32(uint256(2));
        leaves[2] = bytes32(uint256(3));
        leaves[3] = bytes32(uint256(4));

        bytes32 root = SetupPaymentsLib.merkleizeKeccak(leaves);
        assertNotEq(root, bytes32(0), "Merkle root should not be zero");
    }
}