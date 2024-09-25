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
    function createPaymentSubmissions(uint256 numPayments, uint256 amountPerPayment, uint32 duration) internal {

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

    function submitPaymentRoot(address[] calldata earners) public {
        bytes32 paymentRoot = createPaymentRoot(earners);
        uint32 rewardsCalculationEndTimestamp = IRewardsCoordinator(coreDeployment.rewardsCoordinator).currRewardsCalculationEndTimestamp() + 1 weeks;
        IRewardsCoordinator(coreDeployment.rewardsCoordinator).submitRoot(paymentRoot, rewardsCalculationEndTimestamp);
    }

    //creates the root of the payment tree by creating leaves and merkleizing them
    function createPaymentRoot(address[] calldata earners) public returns(bytes32) {
        require(earners.length == NUM_PAYMENTS, "Number of earners must match number of payments");
        bytes32[] memory leaves = new bytes32[](NUM_PAYMENTS);
        for (uint256 i = 0; i < NUM_PAYMENTS; i++) {
            IRewardsCoordinator.EarnerTreeMerkleLeaf memory leaf = IRewardsCoordinator.EarnerTreeMerkleLeaf({
                earner: earners[i],
                earnerTokenRoot: createTokenRoot()
            });
        }
        // _writeLeavesToJson(leaves, "earner-leaves.json");
        return merkleizeSha256(leaves);
    }

    //create individual payment leaves' token root that goes into earner leaf
    function createTokenRoot() public returns(bytes32) {
        bytes32[] memory leaves = new bytes32[](NUM_TOKEN_EARNINGS);
        for (uint256 i = 0; i < NUM_TOKEN_EARNINGS; i++) {
            IRewardsCoordinator.TokenTreeMerkleLeaf memory leaf = IRewardsCoordinator.TokenTreeMerkleLeaf({
                token: IStrategy(helloWorldDeployment.strategy).underlyingToken(),
                cumulativeEarnings: TOKEN_EARNINGS
            });
            leaves[i] = IRewardsCoordinator(coreDeployment.rewardsCoordinator).calculateTokenLeafHash(leaf);
        }

        return merkleizeKeccak(leaves);   
    }

    function _writeLeavesToJson(bytes32[] memory leaves, string memory path) internal {
        string memory leavesJson = "[";
        for (uint256 i = 0; i < leaves.length; i++) {
            leavesJson = string.concat(leavesJson, "\"", leaves[i].toHexString(), "\"");
            if (i != leaves.length - 1) {
                leavesJson = string.concat(leavesJson, ",");
            }
        }
        leavesJson = string.concat(leavesJson, "]");
        vm.writeFile(path, leavesJson);
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
