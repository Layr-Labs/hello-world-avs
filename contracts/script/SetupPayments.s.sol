// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {HelloWorldDeploymentLib} from "./utils/HelloWorldDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {Merkle} from "@eigenlayer/contracts/libraries/Merkle.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IStrategy} from "eigenlayer-contracts/src/contracts/interfaces/IStrategyManager.sol";


contract SetupPayments is Script {

    address private deployer;
    address proxyAdmin;
    CoreDeploymentLib.DeploymentData coreDeployment;
    HelloWorldDeploymentLib.DeploymentData helloWorldDeployment;

    uint256 NUM_PAYMENTS = 8;
    uint256 NUM_TOKEN_EARNINGS = 1;
    uint256 TOKEN_EARNINGS = 100;
    
    struct PaymentLeaves{
        bytes32[] leaves;
        bytes32[] tokenLeaves;
    }

    function setUp() public {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");

        coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);
        helloWorldDeployment = HelloWorldDeploymentLib.readDeploymentJson("deployments/hello-world/", block.chainid);
    }

    function run() external {
        vm.startBroadcast(deployer);
        //set rewardsUpdater to be the deployer
        IRewardsCoordinator(coreDeployment.rewardsCoordinator).setRewardsUpdater(deployer);

        vm.stopBroadcast();
    }
    //submits the payments on behalf of the AVS
    function createPaymentSubmissions(uint256 numPayments, uint256 amountPerPayment, uint32 duration) public {

        IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions = new IRewardsCoordinator.RewardsSubmission[](numPayments);
        for (uint256 i = 0; i < numPayments; i++) {
            IRewardsCoordinator.StrategyAndMultiplier[] memory strategiesAndMultipliers = new IRewardsCoordinator.StrategyAndMultiplier[](1);
            strategiesAndMultipliers[0] = IRewardsCoordinator.StrategyAndMultiplier({
                strategy: IStrategy(helloWorldDeployment.strategy),
                multiplier: 10000
            });

            IRewardsCoordinator.RewardsSubmission memory rewardsSubmission = IRewardsCoordinator.RewardsSubmission({
                strategiesAndMultipliers: strategiesAndMultipliers,
                token: IStrategy(helloWorldDeployment.strategy).underlyingToken(),
                amount: amountPerPayment,
                startTimestamp: uint32(block.timestamp),
                duration: duration
            });

            rewardsSubmissions[i] = rewardsSubmission;
        }

        IRewardsCoordinator(coreDeployment.rewardsCoordinator).createAVSRewardsSubmission(rewardsSubmissions);
    }

    function processClaim(string memory filePath, uint256 indexToProve, address recipient, IRewardsCoordinator.EarnerTreeMerkleLeaf earnerLeaf) public {
        
        PaymentLeaves memory paymentLeaves = parseLeavesFromJson(filePath);
        
        bytes memory proof = generateMerkleProof(paymentLeaves.leaves, indexToProve);
        //we assuming only 1 token for now, default index is 0
        bytes memory tokenProof = generateMerkleProof(paymentLeaves.tokenLeaves, 0);

        uint32[] memory tokenIndices = new uint32[](NUM_TOKEN_EARNINGS);
        bytes[] memory tokenProofs = new bytes[](NUM_TOKEN_EARNINGS);
        tokenProofs[0] = tokenProof;


        IRewardsCoordinator.RewardsClaim memory claim = IRewardsCoordinator.RewardsClaim({
            rootIndex: 0,
            earnerIndex: indexToProve,
            earnerTreeProof: proof,
            earnerLeaf: earnerLeaf,
            tokenIndices: tokenIndices,
            tokenTreeProofs: tokenProofs,
            tokenLeaves: tokenLeaves
        });

        IRewardsCoordinator(coreDeployment.rewardsCoordinator).processRewardsClaim(claim, recipient);

    }

    function submitPaymentRoot(address[] calldata earners) public {
        bytes32 paymentRoot = createPaymentRoot(earners);
        uint32 rewardsCalculationEndTimestamp = IRewardsCoordinator(coreDeployment.rewardsCoordinator).currRewardsCalculationEndTimestamp() + 1 weeks;
        IRewardsCoordinator(coreDeployment.rewardsCoordinator).submitRoot(paymentRoot, rewardsCalculationEndTimestamp);
    }

    //creates the root of the payment tree by creating leaves and merkleizing them
    function createPaymentRoot(address[] calldata earners) public returns(bytes32) {
        require(earners.length == NUM_PAYMENTS, "Number of earners must match number of payments");
        bytes32[] memory leaves = new bytes32[](NUM_PAYMENTS);
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves = new IRewardsCoordinator.EarnerTreeMerkleLeaf[](NUM_PAYMENTS);
        
        //every entry in the payment tree will have the same token leaves/token root in this example
        bytes32[] memory tokenLeaves = createTokenLeaves();
        for (uint256 i = 0; i < NUM_PAYMENTS; i++) {
            IRewardsCoordinator.EarnerTreeMerkleLeaf memory leaf = IRewardsCoordinator.EarnerTreeMerkleLeaf({
                earner: earners[i],
                earnerTokenRoot: createTokenRoot(tokenLeaves)
            });
            leaves[i] = IRewardsCoordinator(coreDeployment.rewardsCoordinator).calculateEarnerLeafHash(leaf);
            earnerLeaves[i] = leaf;
        }
        writeLeavesToJson(leaves, tokenLeaves);
        return merkleizeSha256(leaves);
    }

    //create individual payment leaves' token root that goes into earner leaf
    function createTokenRoot(bytes32[] calldata tokenLeaves) public returns(bytes32) {
        return merkleizeKeccak(tokenLeaves);   
    }

    function createTokenLeaves() public returns(bytes32[] memory) {
        bytes32[] memory leaves = new bytes32[](NUM_TOKEN_EARNINGS);
        for (uint256 i = 0; i < NUM_TOKEN_EARNINGS; i++) {
            IRewardsCoordinator.TokenTreeMerkleLeaf memory leaf = IRewardsCoordinator.TokenTreeMerkleLeaf({
                token: IStrategy(helloWorldDeployment.strategy).underlyingToken(),
                cumulativeEarnings: TOKEN_EARNINGS
            });
            leaves[i] = IRewardsCoordinator(coreDeployment.rewardsCoordinator).calculateTokenLeafHash(leaf);
        }
        return leaves;
    }


    function writeLeavesToJson(bytes32[] memory leaves, bytes32[] memory tokenLeaves) public {
        string memory parent_object = "parent_object";
        vm.serialize(parent_object, "leaves", leaves);
        string memory finalJson = vm.serialize(parent_object, "tokenLeaves", tokenLeaves);
        vm.writeJson(finalJson, "payments.json");
    }

    function parseLeavesFromJson(string memory filePath) public returns(PaymentLeaves memory) {
        string memory json = vm.readFile(filePath);
        bytes memory data = vm.parseJson(json);
        return abi.decode(data, (PaymentLeaves));
    }



    function generateMerkleProof(bytes32[] memory leaves, uint256 index) internal pure returns (bytes memory) {
        require(leaves.length > 0, "Leaves array cannot be empty");
        require(index < leaves.length, "Index out of bounds");

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

    function merkleizeKeccak(bytes32[] memory leaves) internal pure returns (bytes32) {
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

}
