// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.0;

// import {Script} from "forge-std/Script.sol";
// import {HelloWorldDeploymentLib} from "./utils/HelloWorldDeploymentLib.sol";
// import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
// import {SetupPaymentsLib} from "./utils/SetupPaymentsLib.sol";
// import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";

// contract SetupPayments is Script {
//     address private deployer;
//     CoreDeploymentLib.DeploymentData coreDeployment;
//     HelloWorldDeploymentLib.DeploymentData helloWorldDeployment;

//     uint256 constant NUM_PAYMENTS = 8;
//     uint256 constant NUM_TOKEN_EARNINGS = 1;
//     uint256 constant TOKEN_EARNINGS = 100;

//     function setUp() public {
//         deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
//         vm.label(deployer, "Deployer");

//         coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);
//         helloWorldDeployment = HelloWorldDeploymentLib.readDeploymentJson("deployments/hello-world/", block.chainid);
//     }

//     function run() external {
//         vm.startBroadcast(deployer);
//         IRewardsCoordinator(coreDeployment.rewardsCoordinator).setRewardsUpdater(deployer);


//         vm.stopBroadcast();
//     }


//     function createPaymentSubmissions(uint256 numPayments, uint256 amountPerPayment, uint32 duration, uint32 startTimestamp) public {
//         SetupPaymentsLib.createPaymentSubmissions(
//             IRewardsCoordinator(coreDeployment.rewardsCoordinator),
//             helloWorldDeployment.strategy,
//             numPayments,
//             amountPerPayment,
//             duration,
//             startTimestamp
//         );
//     }

//     function processClaim(string memory filePath, uint256 indexToProve, address recipient, IRewardsCoordinator.EarnerTreeMerkleLeaf calldata earnerLeaf) public {
//         SetupPaymentsLib.processClaim(
//             IRewardsCoordinator(coreDeployment.rewardsCoordinator),
//             filePath,
//             indexToProve,
//             recipient,
//             earnerLeaf,
//             NUM_TOKEN_EARNINGS,
//             helloWorldDeployment.strategy
//         );
//     }

//     function submitPaymentRoot(address[] calldata earners, uint32 endTimestamp) public {
//         SetupPaymentsLib.submitRoot(
//             IRewardsCoordinator(coreDeployment.rewardsCoordinator),
//             earners,
//             helloWorldDeployment.strategy,
//             endTimestamp,
//             NUM_PAYMENTS, 
//             NUM_TOKEN_EARNINGS,
//             TOKEN_EARNINGS
//         );
//     }

//     function createPaymentRoot(address[] calldata earners) public returns (bytes32) {
//         return SetupPaymentsLib.createPaymentRoot(
//             IRewardsCoordinator(coreDeployment.rewardsCoordinator),
//             earners,
//             NUM_PAYMENTS,
//             NUM_TOKEN_EARNINGS,
//             TOKEN_EARNINGS,
//             helloWorldDeployment.strategy
//         );
//     }
// }