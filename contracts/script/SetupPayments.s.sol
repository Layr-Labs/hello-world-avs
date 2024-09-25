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

        return Merkle.merkleizeSha256(leaves);
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

        return Merkle.merkleizeSha256(leaves);   
    }

}
