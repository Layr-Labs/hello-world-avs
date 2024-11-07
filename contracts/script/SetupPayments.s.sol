// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {HelloWorldDeploymentLib} from "./utils/HelloWorldDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {SetupPaymentsLib} from "./utils/SetupPaymentsLib.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";

import "forge-std/Test.sol";

contract SetupPayments is Script, Test {
    struct PaymentInfo { 
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
    string internal constant filePath = "test/mockData/scratch/payment_info.json";

    uint256 constant NUM_TOKEN_EARNINGS = 1;
    uint256 constant DURATION = 1 weeks;
    uint256 constant NUM_EARNERS = 4;
    uint256 constant TOKEN_EARNINGS = 100;


    function setUp() public {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");

        coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);
        helloWorldDeployment = HelloWorldDeploymentLib.readDeploymentJson("deployments/hello-world/", block.chainid);

        // TODO: Get the filePath from config
    }

    function run() external {
        vm.startBroadcast(deployer);
        emit log_named_address("RewardsCoordinator", coreDeployment.rewardsCoordinator);
        IRewardsCoordinator(coreDeployment.rewardsCoordinator).setRewardsUpdater(deployer);
        // PaymentInfo memory info = abi.decode(vm.parseJson(vm.readFile(filePath)), (PaymentInfo));
        // (address[] memory earners, IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves) = _getEarnerAndEarnerLeaves(helloWorldDeployment.strategy);


        // createAVSRewardsSubmissions(info.numPayments, info.amountPerPayment, info.duration, info.startTimestamp);
        // submitPaymentRoot(earners, info.endTimestamp, uint32(info.numPayments), uint32(info.amountPerPayment));

        // processClaim(filePath, info.indexToProve, info.recipient, earnerLeaves[info.indexToProve]);

        vm.stopBroadcast();
    }


    function createAVSRewardsSubmissions(uint256 numPayments, uint256 amountPerPayment, uint32 duration, uint32 startTimestamp) public {
        SetupPaymentsLib.createAVSRewardsSubmissions(
            helloWorldDeployment.helloWorldServiceManager,
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

    function _getEarnerAndEarnerLeaves(address strategy) internal returns (address[] memory, IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory) {
        address[] memory earners = new address[](NUM_EARNERS);
        for (uint256 i = 0; i < earners.length; i++) {
            earners[i] = address(1);
        }
        bytes32[] memory tokenLeaves = SetupPaymentsLib.createTokenLeaves(IRewardsCoordinator(coreDeployment.rewardsCoordinator), NUM_TOKEN_EARNINGS, TOKEN_EARNINGS, strategy);
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves =SetupPaymentsLib.createEarnerLeaves(earners, tokenLeaves);

        return (earners, earnerLeaves);
    }
}