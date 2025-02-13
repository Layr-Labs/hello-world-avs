// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {IStrategy} from "eigenlayer-contracts/src/contracts/interfaces/IStrategyManager.sol";
import {ECDSAServiceManagerBase} from "@eigenlayer-middleware/src/unaudited/ECDSAServiceManagerBase.sol";
import {Vm} from "forge-std/Vm.sol";


library SetupPaymentsLib {

    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    struct PaymentLeaves {
        bytes32[] leaves;
        bytes32[] tokenLeaves;
    }

    function createAVSRewardsSubmissions(
        address helloWorldServiceManager,
        address strategy,
        uint256 numPayments,
        uint256 amountPerPayment,
        uint32 duration,
        uint32 startTimestamp
    ) internal {
        IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions = new IRewardsCoordinator.RewardsSubmission[](numPayments);
        for (uint256 i = 0; i < numPayments; i++) {
            IRewardsCoordinator.StrategyAndMultiplier[] memory strategiesAndMultipliers = new IRewardsCoordinator.StrategyAndMultiplier[](1);
            strategiesAndMultipliers[0] = IRewardsCoordinator.StrategyAndMultiplier({
                strategy: IStrategy(strategy),
                multiplier: 10000
            });

            IRewardsCoordinator.RewardsSubmission memory rewardsSubmission = IRewardsCoordinator.RewardsSubmission({
                strategiesAndMultipliers: strategiesAndMultipliers,
                token: IStrategy(strategy).underlyingToken(),
                amount: amountPerPayment,
                startTimestamp: startTimestamp,
                duration: duration
            });

            rewardsSubmissions[i] = rewardsSubmission;
        }
        ECDSAServiceManagerBase(helloWorldServiceManager).createAVSRewardsSubmission(rewardsSubmissions);
    }
    
    /**
     * @notice Creates Operator Directed AVS rewards submissions
     * @dev Helper function that creates standard AVS rewards submissions with single strategy
     * @param helloWorldServiceManager Address of the service manager contract
     * @param strategy Address of the strategy contract
     * @param numPayments Number of payment submissions to create
     * @param amountPerPayment Amount of tokens per payment submission
     * @param duration Duration in seconds for the rewards period
     * @param startTimestamp Start timestamp for the rewards period
     */
    function createOperatorDirectedAVSRewardsSubmissions(
        address helloWorldServiceManager,
        address[] memory operators,
        uint256 numOperators,
        address strategy,
        uint256 numPayments,
        uint256 amountPerPayment,
        uint32 duration,
        uint32 startTimestamp
    ) internal {

        uint256 operatorRewardAmount = amountPerPayment / numOperators;

        IRewardsCoordinator.OperatorReward[] memory operatorRewards = new IRewardsCoordinator.OperatorReward[](2);
        for (uint256 i = 0; i < 2; i++) {
            operatorRewards[i] = IRewardsCoordinator.OperatorReward({
                operator: operators[i],
                amount: operatorRewardAmount
            });
        }

        IRewardsCoordinator.OperatorDirectedRewardsSubmission[] memory rewardsSubmissions = new IRewardsCoordinator.OperatorDirectedRewardsSubmission[](numPayments);
        for (uint256 i = 0; i < numPayments; i++) {
            IRewardsCoordinator.StrategyAndMultiplier[] memory strategiesAndMultipliers = new IRewardsCoordinator.StrategyAndMultiplier[](1);
            strategiesAndMultipliers[0] = IRewardsCoordinator.StrategyAndMultiplier({
                strategy: IStrategy(strategy),
                multiplier: 10000
            });

            IRewardsCoordinator.OperatorDirectedRewardsSubmission memory rewardsSubmission = IRewardsCoordinator.OperatorDirectedRewardsSubmission({
                strategiesAndMultipliers: strategiesAndMultipliers,
                token: IStrategy(strategy).underlyingToken(),
                operatorRewards: operatorRewards,
                startTimestamp: startTimestamp,
                duration: duration,
                description: "test"
            });

            rewardsSubmissions[i] = rewardsSubmission;
        }
        ECDSAServiceManagerBase(helloWorldServiceManager).createOperatorDirectedAVSRewardsSubmission(rewardsSubmissions);
    }

    /**
     * @notice Process a rewards claim for a recipient
     * @param rewardsCoordinator The RewardsCoordinator contract instance
     * @param filePath Path to the JSON file containing merkle leaves
     * @param indexToProve Index of the earner leaf to prove
     * @param recipient Address to receive the rewards
     * @param earnerLeaf The earner leaf data structure containing proof details
     * @param NUM_TOKEN_EARNINGS Number of token earnings to process
     * @param strategy The strategy contract address
     * @param amountPerPayment Amount of tokens per payment
     */
    function processClaim(
        IRewardsCoordinator rewardsCoordinator,
        string memory filePath,
        uint256 indexToProve,
        address recipient,
        IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf,
        uint256 NUM_TOKEN_EARNINGS,
        address strategy,
        uint32 amountPerPayment
    ) internal {
        PaymentLeaves memory paymentLeaves = parseLeavesFromJson(filePath);
        
        bytes memory proof = generateMerkleProof(paymentLeaves.leaves, indexToProve);
        //we only have one token leaf
        bytes memory tokenProof = generateMerkleProof(paymentLeaves.tokenLeaves, 0);

        uint32[] memory tokenIndices = new uint32[](NUM_TOKEN_EARNINGS);
        bytes[] memory tokenProofs = new bytes[](NUM_TOKEN_EARNINGS);
        tokenProofs[0] = tokenProof;

        IRewardsCoordinator.TokenTreeMerkleLeaf[] memory tokenLeaves = new IRewardsCoordinator.TokenTreeMerkleLeaf[](NUM_TOKEN_EARNINGS);
        tokenLeaves[0] = defaultTokenLeaf(amountPerPayment, strategy);


        // this workflow assumes a new root submitted for every payment claimed.  So we get the latest rood index to process a claim for
        uint256 rootIndex = rewardsCoordinator.getDistributionRootsLength() - 1;

        IRewardsCoordinator.RewardsMerkleClaim memory claim = IRewardsCoordinator.RewardsMerkleClaim({
            rootIndex: uint32(rootIndex),
            earnerIndex: uint32(indexToProve),
            earnerTreeProof: proof,
            earnerLeaf: earnerLeaf,
            tokenIndices: tokenIndices,
            tokenTreeProofs: tokenProofs,
            tokenLeaves: tokenLeaves
        });

        rewardsCoordinator.processClaim(claim, recipient);
    }

    /**
     * @notice Creates token leaves for the rewards merkle tree
     * @dev Each token leaf represents a token payment amount and strategy
     * @dev This is a helper function used to simulate payment root submission for testing.
     *      In production, payment roots are managed by an off-chain data pipeline.
     *      See: https://github.com/Layr-Labs/eigenlayer-contracts/blob/dev/docs/core/RewardsCoordinator.md#off-chain-calculation
     
     */
    function submitRoot(
        IRewardsCoordinator rewardsCoordinator,
        bytes32[] memory tokenLeaves,
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves,
        address strategy,
        uint32 rewardsCalculationEndTimestamp,
         uint256 NUM_PAYMENTS,
        uint256 NUM_TOKEN_EARNINGS,
        string memory filePath
    ) internal {
        bytes32 paymentRoot = createPaymentRoot(rewardsCoordinator, tokenLeaves, earnerLeaves, NUM_PAYMENTS, NUM_TOKEN_EARNINGS, filePath);
        rewardsCoordinator.submitRoot(paymentRoot, rewardsCalculationEndTimestamp);
    }

    /**
     * @notice Creates AVS rewards submissions for operators
     * @dev This is a helper function used to simulate AVS rewards submissions for testing.
     *      In production, rewards submissions are managed by an off-chain data pipeline.
     * @param helloWorldServiceManager The HelloWorldServiceManager contract address
     * @param strategy The strategy contract address 
     * @param numPayments The number of payments to create
     * @param amountPerPayment The amount of tokens per payment
     * @param duration The duration of the rewards period
     * @param startTimestamp The timestamp when the rewards period starts
     */
    function createAVSRewardsSubmissions(
        address helloWorldServiceManager,
        address strategy,
        uint256 numPayments,
        uint256 amountPerPayment,
        uint32 duration,
        uint32 startTimestamp
    ) public {
        for (uint256 i = 0; i < numPayments; i++) {
            IHelloWorldServiceManager(helloWorldServiceManager).submitAVSRewardsSubmission(
                strategy,
                amountPerPayment,
                duration,
                startTimestamp
            );
        }
    }

    /**
     * @notice Creates operator-directed AVS rewards submissions
     * @dev This is a helper function used to simulate operator-directed AVS rewards submissions for testing.
     *      In production, rewards submissions are managed by an off-chain data pipeline.
     * @param helloWorldServiceManager The HelloWorldServiceManager contract address
     * @param operators Array of operator addresses to receive rewards
     * @param numOperators Number of operators in the operators array
     * @param strategy The strategy contract address
     * @param numPayments The number of payments to create
     * @param amountPerPayment The amount of tokens per payment
     * @param duration The duration of the rewards period
     * @param startTimestamp The timestamp when the rewards period starts
     */
    function createOperatorDirectedAVSRewardsSubmissions(
        address helloWorldServiceManager,
        address[] memory operators,
        uint256 numOperators,
        address strategy,
        uint256 numPayments,
        uint256 amountPerPayment,
        uint32 duration,
        uint32 startTimestamp
    ) public {
        for (uint256 i = 0; i < numPayments; i++) {
            for (uint256 j = 0; j < numOperators; j++) {
                IHelloWorldServiceManager(helloWorldServiceManager).submitOperatorDirectedAVSRewardsSubmission(
                    operators[j],
                    strategy,
                    amountPerPayment,
                    duration,
                    startTimestamp
                );
            }
        }
    }
    
    /**
     * @notice Creates token leaves for the rewards merkle tree
     * @dev This is a helper function used to create token leaves for testing.
     *      In production, token leaves are managed by an off-chain data pipeline.
     *      See: https://github.com/Layr-Labs/eigenlayer-contracts/blob/dev/docs/core/RewardsCoordinator.md#off-chain-calculation
     * @param rewardsCoordinator The RewardsCoordinator contract interface
     * @param numTokenEarnings The number of token earnings to create leaves for
     * @param amountPerPayment The amount of tokens per payment
     * @param strategy The strategy contract address
     * @return Array of token leaf hashes
     */
    function createPaymentRoot(
        IRewardsCoordinator rewardsCoordinator,
        bytes32[] memory tokenLeaves,
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves,
        uint256 NUM_PAYMENTS,
        uint256 NUM_TOKEN_EARNINGS,
        string memory filePath
    ) internal returns (bytes32) {
        require(earnerLeaves.length == NUM_PAYMENTS, "Number of earners must match number of payments");
        bytes32[] memory leaves = new bytes32[](NUM_PAYMENTS);
        
        require(tokenLeaves.length == NUM_TOKEN_EARNINGS, "Number of token leaves must match number of token earnings");
        for (uint256 i = 0; i < NUM_PAYMENTS; i++) {
            leaves[i] = rewardsCoordinator.calculateEarnerLeafHash(earnerLeaves[i]);
        }

        writeLeavesToJson(leaves, tokenLeaves, filePath);
        return (merkleizeKeccak(leaves));
    }

    /**
     * @notice Creates earner leaves for the rewards merkle tree
     * @param earners Array of earner addresses
     * @param tokenLeaves Array of token leaf hashes
     * @return Array of EarnerTreeMerkleLeaf structures
     */
    function createEarnerLeaves(
        address[] calldata earners,
        bytes32[] memory tokenLeaves
    ) public pure returns (IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory) {
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory leaves = new IRewardsCoordinator.EarnerTreeMerkleLeaf[](earners.length);
        for (uint256 i = 0; i < earners.length; i++) {
            leaves[i] = IRewardsCoordinator.EarnerTreeMerkleLeaf({
                earner: earners[i],
                earnerTokenRoot: createTokenRoot(tokenLeaves)
            });
        }
        return leaves;
    }

    /**
     * @notice Creates a token root from token leaves
     * @param tokenLeaves Array of token leaf hashes
     * @return bytes32 The calculated token root
     */
    function createTokenRoot(bytes32[] memory tokenLeaves) public pure returns (bytes32) {
        return merkleizeKeccak(tokenLeaves);   
    }

    /**
     * @notice Creates token leaves for the rewards merkle tree
     * @param rewardsCoordinator The RewardsCoordinator contract instance
     * @param NUM_TOKEN_EARNINGS Number of token earnings to create
     * @param TOKEN_EARNINGS Amount of tokens per earning
     * @param strategy The strategy contract address
     * @return Array of token leaf hashes
     */
    function createTokenLeaves(
        IRewardsCoordinator rewardsCoordinator,
        uint256 NUM_TOKEN_EARNINGS,
        uint256 TOKEN_EARNINGS,
        address strategy
    ) internal view returns (bytes32[] memory) {
        bytes32[] memory leaves = new bytes32[](NUM_TOKEN_EARNINGS);
        for (uint256 i = 0; i < NUM_TOKEN_EARNINGS; i++) {
            IRewardsCoordinator.TokenTreeMerkleLeaf memory leaf = defaultTokenLeaf(TOKEN_EARNINGS, strategy);
            leaves[i] = rewardsCoordinator.calculateTokenLeafHash(leaf);
        }
        return leaves;
    }

    /**
     * @notice Creates a default token leaf structure
     * @param TOKEN_EARNINGS Amount of tokens for this leaf
     * @param strategy The strategy contract address
     * @return TokenTreeMerkleLeaf The created token leaf structure
     */
    function defaultTokenLeaf(
        uint256 TOKEN_EARNINGS,
        address strategy
    ) internal view returns (IRewardsCoordinator.TokenTreeMerkleLeaf memory) {
        IRewardsCoordinator.TokenTreeMerkleLeaf memory leaf = IRewardsCoordinator.TokenTreeMerkleLeaf({
            token: IStrategy(strategy).underlyingToken(),
            cumulativeEarnings: TOKEN_EARNINGS
        });
        return leaf;
    }

    /**
     * @notice Writes merkle leaves to a JSON file
     * @param leaves Array of earner leaf hashes
     * @param tokenLeaves Array of token leaf hashes
     * @param filePath Path where the JSON file will be written
     */
    function writeLeavesToJson(
        bytes32[] memory leaves,
        bytes32[] memory tokenLeaves,
        string memory filePath
    ) internal {
        string memory parent_object = "parent_object";
        vm.serializeBytes32(parent_object, "leaves", leaves);
        string memory finalJson = vm.serializeBytes32(parent_object, "tokenLeaves", tokenLeaves);
        vm.writeJson(finalJson, filePath);
    }

    /**
     * @notice Reads merkle leaves from a JSON file
     * @param filePath Path to the JSON file
     * @return PaymentLeaves structure containing the leaves and token leaves
     */
    function parseLeavesFromJson(string memory filePath) internal view returns (PaymentLeaves memory) {
        string memory json = vm.readFile(filePath);
        bytes memory data = vm.parseJson(json);
        return abi.decode(data, (PaymentLeaves));
    }

    /**
     * @notice Generates a merkle proof for a given leaf index
     * @param leaves Array of leaf hashes
     * @param index Index of the leaf to generate proof for
     * @return bytes The generated merkle proof
     */
    function generateMerkleProof(bytes32[] memory leaves, uint256 index) internal pure returns (bytes memory) {
        require(leaves.length > 0, "Leaves array cannot be empty");
        require(index < leaves.length, "Index out of bounds");

        leaves = padLeaves(leaves);

        uint256 n = leaves.length;
        uint256 depth = 0;
        while ((1 << depth) < n) {
            depth++;
        }

        bytes32[] memory proof = new bytes32[](depth);
        uint256 proofIndex = 0;

        for (uint256 i = 0; i < depth; i++) {
            uint256 levelSize = (n + 1) / 2;
            uint256 siblingIndex = (index % 2 == 0) ? index + 1 : index - 1;

            if (siblingIndex < n) {
                proof[proofIndex] = leaves[siblingIndex];
                proofIndex++;
            }

            for (uint256 j = 0; j < levelSize; j++) {
                if (2 * j + 1 < n) {
                    leaves[j] = keccak256(abi.encodePacked(leaves[2 * j], leaves[2 * j + 1]));
                } else {
                    leaves[j] = leaves[2 * j];
                }
            }

            n = levelSize;
            index /= 2;
        }

        return abi.encodePacked(proof);
    }

    /**
     * @notice Pads an array of leaves to the next power of 2
     * @param leaves Array of leaf hashes to pad
     * @return bytes32[] The padded array of leaves
     */
    function padLeaves(bytes32[] memory leaves) internal pure returns (bytes32[] memory) {
        uint256 paddedLength = 2;
        while(paddedLength < leaves.length) {
            paddedLength <<= 1;
        }

        bytes32[] memory paddedLeaves = new bytes32[](paddedLength);
        for (uint256 i = 0; i < leaves.length; i++) {
            paddedLeaves[i] = leaves[i];
        }
        return paddedLeaves;
    }

    /**
     * @notice this function returns the merkle root of a tree created from a set of leaves using keccak256 as its hash function
     *  @param leaves the leaves of the merkle tree
     *  @return The computed Merkle root of the tree.
     *  @dev This pads to the next power of 2. very inefficient! just for POC
     */
    function merkleizeKeccak(bytes32[] memory leaves) internal pure returns (bytes32) {
        // uint256 paddedLength = 2;
        // while(paddedLength < leaves.length) {
        //     paddedLength <<= 1;
        // }

        // bytes32[] memory paddedLeaves = new bytes32[](paddedLength);
        // for (uint256 i = 0; i < leaves.length; i++) {
        //     paddedLeaves[i] = leaves[i];
        // }
        leaves = padLeaves(leaves);

        //there are half as many nodes in the layer above the leaves
        uint256 numNodesInLayer = leaves.length / 2;
        //create a layer to store the internal nodes
        bytes32[] memory layer = new bytes32[](numNodesInLayer);
        //fill the layer with the pairwise hashes of the leaves
        for (uint256 i = 0; i < numNodesInLayer; i++) {
            layer[i] = keccak256(abi.encodePacked(leaves[2 * i], leaves[2 * i + 1]));
        }
        //the next layer above has half as many nodes
        numNodesInLayer /= 2;
        //while we haven't computed the root
        while (numNodesInLayer != 0) {
            //overwrite the first numNodesInLayer nodes in layer with the pairwise hashes of their children
            for (uint256 i = 0; i < numNodesInLayer; i++) {
                layer[i] = keccak256(abi.encodePacked(layer[2 * i], layer[2 * i + 1]));
            }
            //the next layer above has half as many nodes
            numNodesInLayer /= 2;
        }
        //the first node in the layer is the root
        return layer[0];
    }

    function padLeaves(bytes32[] memory leaves) internal pure returns (bytes32[] memory) {
        uint256 paddedLength = 2;
        while(paddedLength < leaves.length) {
            paddedLength <<= 1;
        }

        bytes32[] memory paddedLeaves = new bytes32[](paddedLength);
        for (uint256 i = 0; i < leaves.length; i++) {
            paddedLeaves[i] = leaves[i];
        }
        return paddedLeaves;
    }
}