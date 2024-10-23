// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {HelloWorldDeploymentLib} from "./utils/HelloWorldDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {SetupPaymentsLib} from "./utils/SetupPaymentsLib.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";

contract SetupPayments is Script {
    struct PaymentInfo {
        address[] earners;
        bytes32[] earnerTokenRoots;
        address recipient;
        uint256 numPayments;
        uint256 amountPerPayment;
        uint32 duration;
        uint32 startTimestamp;
        uint32 endTimestamp;
        uint256 indexToProve;
    }

    address private deployer;
    CoreDeploymentLib.DeploymentData coreDeployment;
    HelloWorldDeploymentLib.DeploymentData helloWorldDeployment;
    string internal constant filePath = "test/mockData/scratch/payments.json";

    uint256 constant NUM_TOKEN_EARNINGS = 1;
    uint256 constant DURATION = 1 weeks;

    function setUp() public {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");

        coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);
        helloWorldDeployment = HelloWorldDeploymentLib.readDeploymentJson("deployments/hello-world/", block.chainid);

        // TODO: Get the filePath from config
    }

    function run() external {
        vm.startBroadcast(deployer);
        IRewardsCoordinator(coreDeployment.rewardsCoordinator).setRewardsUpdater(deployer);
        PaymentInfo memory info = abi.decode(vm.parseJson(vm.readFile(filePath)), (PaymentInfo));

        createAVSRewardsSubmissions(info.numPayments, info.amountPerPayment, info.duration, info.startTimestamp);
        submitPaymentRoot(info.earners, info.endTimestamp, uint32(info.numPayments), uint32(info.amountPerPayment));

        IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf = IRewardsCoordinator.EarnerTreeMerkleLeaf({
            earner: info.earners[info.indexToProve],
            earnerTokenRoot: info.earnerTokenRoots[info.indexToProve]
        });

        processClaim(filePath, info.indexToProve, info.recipient, earnerLeaf);

        vm.stopBroadcast();
    }


    function createAVSRewardsSubmissions(uint256 numPayments, uint256 amountPerPayment, uint32 duration, uint32 startTimestamp) public {
        SetupPaymentsLib.createAVSRewardsSubmissions(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            helloWorldDeployment.strategy,
            numPayments,
            amountPerPayment,
            duration,
            startTimestamp
        );
    }

    function processClaim(string memory filePath, uint256 indexToProve, address recipient, IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf) public {
        SetupPaymentsLib.processClaim(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            filePath,
            indexToProve,
            recipient,
            earnerLeaf,
            NUM_TOKEN_EARNINGS,
            helloWorldDeployment.strategy
        );
    }

    function submitPaymentRoot(address[] memory earners, uint32 endTimestamp, uint32 numPayments, uint32 amountPerPayment) public {
        bytes32[] memory tokenLeaves = SetupPaymentsLib.createTokenLeaves(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator), 
            NUM_TOKEN_EARNINGS, 
            amountPerPayment, 
            helloWorldDeployment.strategy
        );
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves = SetupPaymentsLib.createEarnerLeaves(earners, tokenLeaves);

        SetupPaymentsLib.submitRoot(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            tokenLeaves,
            earnerLeaves,
            helloWorldDeployment.strategy,
            endTimestamp,
            numPayments, 
            NUM_TOKEN_EARNINGS,
            filePath
        );
    }
}