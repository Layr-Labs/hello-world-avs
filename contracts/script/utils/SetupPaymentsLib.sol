// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {IStrategy} from "eigenlayer-contracts/src/contracts/interfaces/IStrategyManager.sol";
import {Vm} from "forge-std/Vm.sol";

library SetupPaymentsLib {
    struct PaymentLeaves {
        bytes32[] leaves;
        bytes32[] tokenLeaves;
    }

    function createPaymentSubmissions(
        IRewardsCoordinator rewardsCoordinator,
        address strategy,
        uint256 numPayments,
        uint256 amountPerPayment,
        uint32 duration
    ) public {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function processClaim(
        IRewardsCoordinator rewardsCoordinator,
        string memory filePath,
        uint256 indexToProve,
        address recipient,
        IRewardsCoordinator.EarnerTreeMerkleLeaf calldata earnerLeaf,
        uint256 NUM_TOKEN_EARNINGS,
        address strategy
    ) public {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function submitPaymentRoot(
        IRewardsCoordinator rewardsCoordinator,
        address[] calldata earners
    ) public {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function createPaymentRoot(
        IRewardsCoordinator rewardsCoordinator,
        address[] calldata earners,
        uint256 NUM_PAYMENTS,
        uint256 NUM_TOKEN_EARNINGS,
        uint256 TOKEN_EARNINGS,
        address strategy,
        Vm vm
    ) public returns (bytes32) {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function createTokenRoot(bytes32[] memory tokenLeaves) public pure returns (bytes32) {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function createTokenLeaves(
        IRewardsCoordinator rewardsCoordinator,
        uint256 NUM_TOKEN_EARNINGS,
        uint256 TOKEN_EARNINGS,
        address strategy
    ) public returns (bytes32[] memory) {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function defaultTokenLeaf(
        uint256 TOKEN_EARNINGS,
        address strategy
    ) public view returns (IRewardsCoordinator.TokenTreeMerkleLeaf memory) {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function writeLeavesToJson(
        bytes32[] memory leaves,
        bytes32[] memory tokenLeaves,
        Vm vm
    ) public {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function parseLeavesFromJson(string memory filePath, Vm vm) public returns (PaymentLeaves memory) {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function generateMerkleProof(bytes32[] memory leaves, uint256 index) internal pure returns (bytes memory) {
        // ... (copy the function body from SetupPayments.s.sol)
    }

    function merkleizeKeccak(bytes32[] memory leaves) internal pure returns (bytes32) {
        // ... (copy the function body from SetupPayments.s.sol)
    }
}
