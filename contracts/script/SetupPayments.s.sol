// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {HelloWorldDeploymentLib} from "./utils/HelloWorldDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {SetupPaymentsLib} from "./utils/SetupPaymentsLib.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {RewardsCoordinator} from "@eigenlayer/contracts/core/RewardsCoordinator.sol";
import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategy.sol";
import {ERC20Mock} from "../test/ERC20Mock.sol";

import "forge-std/Test.sol";

contract SetupPayments is Script, Test {
    struct PaymentInfo { 
        address recipient;
        uint32 numPayments;
        uint32 amountPerPayment;
        uint32 duration;
        uint32 startTimestamp;
        uint32 endTimestamp;
        uint256 indexToProve;
    }

    address private deployer;
    CoreDeploymentLib.DeploymentData coreDeployment;
    CoreDeploymentLib.DeploymentConfigData coreConfig;

    HelloWorldDeploymentLib.DeploymentData helloWorldDeployment;
    HelloWorldDeploymentLib.DeploymentConfigData helloWorldConfig;

    RewardsCoordinator rewardsCoordinator;
    string internal constant paymentInfofilePath = "test/mockData/scratch/payment_info.json";
    string internal constant filePath = "test/mockData/scratch/payments.json";


    
    uint32 constant CALCULATION_INTERVAL_SECONDS = 1 days;
    uint256 constant NUM_TOKEN_EARNINGS = 1;
    uint32 constant DURATION = 1;
    uint256 constant NUM_EARNERS = 8;

    uint32 numPayments = 8;
    uint32 indexToProve = 0;
    uint32 amountPerPayment = 100;

    address recipient = address(1);
    IRewardsCoordinator.EarnerTreeMerkleLeaf[] public earnerLeaves;
    address[] public earners;
    uint32 startTimestamp;
    uint32 endTimestamp;
    uint256 cumumlativePaymentMultiplier;
    address nonceSender = 0x998abeb3E57409262aE5b751f60747921B33613E;

    address operator1 = address(1);
    address operator2 = address(2);


    function setUp() public {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");

        coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);
        coreConfig = CoreDeploymentLib.readDeploymentConfigValues("config/core/", block.chainid);
        helloWorldDeployment = HelloWorldDeploymentLib.readDeploymentJson("deployments/hello-world/", block.chainid);
        helloWorldConfig = HelloWorldDeploymentLib.readDeploymentConfigValues("config/hello-world/", block.chainid);

        rewardsCoordinator = RewardsCoordinator(coreDeployment.rewardsCoordinator);

        // TODO: Get the filePath from config
    }

    function run() external {
        vm.startBroadcast(helloWorldConfig.rewardsInitiatorKey);
    
        if(rewardsCoordinator.currRewardsCalculationEndTimestamp() == 0) {
             startTimestamp = uint32(block.timestamp) - (uint32(block.timestamp) % CALCULATION_INTERVAL_SECONDS);
        } else {
            startTimestamp = rewardsCoordinator.currRewardsCalculationEndTimestamp() - DURATION + CALCULATION_INTERVAL_SECONDS;
        }

        endTimestamp = startTimestamp + 1;


        if (endTimestamp > block.timestamp) {
            revert("End timestamp must be in the future.  Please wait to generate new payments.");
        }

        // sets a multiplier based on block number such that cumulativeEarnings increase accordingly for multiple runs of this script in the same session
        uint256 nonce = rewardsCoordinator.getDistributionRootsLength();
        amountPerPayment = uint32(amountPerPayment * (nonce + 1));
        
        createAVSRewardsSubmissions(numPayments, amountPerPayment, startTimestamp);
        vm.stopBroadcast();
        vm.startBroadcast(deployer);
        earners = _getEarners(deployer);
        submitPaymentRoot(earners, endTimestamp, numPayments, amountPerPayment);
        vm.stopBroadcast();
    }


    /**
     * @notice Create Operator Directed AVS Rewards Submission and submit payment root to the rewards coordinator
     */
    function runOperatorDirected() external {
        vm.startBroadcast(helloWorldConfig.rewardsInitiatorKey);
        if(rewardsCoordinator.currRewardsCalculationEndTimestamp() == 0) {
             startTimestamp = uint32(block.timestamp) - (uint32(block.timestamp) % CALCULATION_INTERVAL_SECONDS);
        } else {
            startTimestamp = rewardsCoordinator.currRewardsCalculationEndTimestamp() - DURATION + CALCULATION_INTERVAL_SECONDS;
        }

        endTimestamp = startTimestamp + 1;


        if (endTimestamp > block.timestamp) {
            revert("End timestamp must be in the future.  Please wait to generate new payments.");
        }

        // sets a multiplier based on block number such that cumulativeEarnings increase accordingly for multiple runs of this script in the same session
        uint256 nonce = rewardsCoordinator.getDistributionRootsLength();
        amountPerPayment = uint32(amountPerPayment * (nonce + 1));

        // Create Operator Directed AVS Rewards Submission
        createOperatorDirectedAVSRewardsSubmissions(numPayments, amountPerPayment, startTimestamp);
        vm.stopBroadcast();

        vm.startBroadcast(deployer);
        earners = _getEarners(deployer);
        submitPaymentRoot(earners, endTimestamp, numPayments, amountPerPayment);
        vm.stopBroadcast();
    }
        

    function executeProcessClaim() public {
        uint256 nonce = rewardsCoordinator.getDistributionRootsLength();
        amountPerPayment = uint32(amountPerPayment * nonce);

        vm.startBroadcast(deployer);
        earnerLeaves = _getEarnerLeaves(_getEarners(deployer), amountPerPayment, helloWorldDeployment.strategy);
        processClaim(filePath, indexToProve, recipient, earnerLeaves[indexToProve], amountPerPayment);
        vm.stopBroadcast();
    }


    /**
     * @notice Creates AVS rewards submissions
     * @dev Helper function that mints tokens, approves spending, and creates AVS rewards submissions
     * @param numPayments The number of payment submissions to create
     * @param amountPerPayment The amount of tokens per payment
     * @param startTimestamp The timestamp when the rewards period starts
     */
    function createAVSRewardsSubmissions(uint256 numPayments, uint256 amountPerPayment, uint32 startTimestamp) public {
        ERC20Mock(helloWorldDeployment.token).mint(helloWorldConfig.rewardsInitiator, amountPerPayment * numPayments);
        ERC20Mock(helloWorldDeployment.token).increaseAllowance(helloWorldDeployment.helloWorldServiceManager, amountPerPayment * numPayments);
        uint32 duration = rewardsCoordinator.MAX_REWARDS_DURATION();
        SetupPaymentsLib.createAVSRewardsSubmissions(
            helloWorldDeployment.helloWorldServiceManager,
            helloWorldDeployment.strategy,
            numPayments,
            amountPerPayment,
            duration,
            startTimestamp
        );
    }

    /**
     * @notice Creates operator-directed AVS rewards submissions
     * @dev Helper function that mints tokens, approves spending, and creates operator-directed rewards submissions
     * @param numPayments The number of payment submissions to create
     * @param amountPerPayment The amount of tokens per payment
     * @param startTimestamp The timestamp when the rewards period starts
     */
    function createOperatorDirectedAVSRewardsSubmissions(uint256 numPayments, uint256 amountPerPayment, uint32 startTimestamp) public {
        ERC20Mock(helloWorldDeployment.token).mint(helloWorldConfig.rewardsInitiator, amountPerPayment * numPayments);
        ERC20Mock(helloWorldDeployment.token).increaseAllowance(helloWorldDeployment.helloWorldServiceManager, amountPerPayment * numPayments);
        uint32 duration = 0;
        address[] memory operators = new address[](2);
        operators[0] = operator1;
        operators[1] = operator2;

        uint256 numOperators = operators.length;

        SetupPaymentsLib.createOperatorDirectedAVSRewardsSubmissions(
            helloWorldDeployment.helloWorldServiceManager,
            operators,
            numOperators,
            helloWorldDeployment.strategy,
            numPayments,
            amountPerPayment,
            duration,
            startTimestamp
        );
    }

    /**
     * @notice Processes a claim for a payment
     * @dev Helper function that processes a claim for a payment
     *      In production, payment roots are managed by an off-chain data pipeline.
     *      See: https://github.com/Layr-Labs/eigenlayer-contracts/blob/dev/docs/core/RewardsCoordinator.md#off-chain-calculation
     * @param filePath The path to the file containing the payment information
     * @param indexToProve The index of the earner to prove
     * @param recipient The address of the recipient
     * @param earnerLeaf The earner leaf to prove
     * @param amountPerPayment The amount of tokens per payment
     */
    function processClaim(string memory filePath, uint256 indexToProve, address recipient, IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf, uint32 amountPerPayment) public {
        SetupPaymentsLib.processClaim(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            filePath,
            indexToProve,
            recipient,
            earnerLeaf,
            NUM_TOKEN_EARNINGS,
            helloWorldDeployment.strategy,
            amountPerPayment
        );
    }

    /**
     * @notice Submits a payment root to the RewardsCoordinator contract
     * @dev This is a helper function used to simulate payment root submission for testing.
     *      In production, payment roots are managed by an off-chain data pipeline.
     *      See: https://github.com/Layr-Labs/eigenlayer-contracts/blob/dev/docs/core/RewardsCoordinator.md#off-chain-calculation
     * @param earners Array of addresses that will receive payments
     * @param endTimestamp The timestamp when the payment period ends
     * @param numPayments The number of payments to be made
     * @param amountPerPayment The amount of tokens per payment
     */

    function submitPaymentRoot(address[] memory earners, uint32 endTimestamp, uint32 numPayments, uint32 amountPerPayment) public {
        emit log_named_uint("cumumlativePaymentMultiplier", cumumlativePaymentMultiplier);
        bytes32[] memory tokenLeaves = SetupPaymentsLib.createTokenLeaves(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator), 
            NUM_TOKEN_EARNINGS, 
            amountPerPayment, 
            helloWorldDeployment.strategy
        );
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves = SetupPaymentsLib.createEarnerLeaves(earners, tokenLeaves);
        emit log_named_uint("Earner Leaves Length", earnerLeaves.length);
        emit log_named_uint("numPayments", numPayments);    

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

    function _getEarnerLeaves(address[] memory earners, uint32 amountPerPayment, address strategy) internal view returns (IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory) {
        bytes32[] memory tokenLeaves = SetupPaymentsLib.createTokenLeaves(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator), 
            NUM_TOKEN_EARNINGS, 
            amountPerPayment, 
            strategy
        );

        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves = SetupPaymentsLib.createEarnerLeaves(earners, tokenLeaves);

        return earnerLeaves;
    }

    function _getEarners(address deployer) internal pure returns (address[] memory) {
        address[] memory earners = new address[](NUM_EARNERS);
        for (uint256 i = 0; i < earners.length; i++) {
            earners[i] = deployer;
        }
        return earners;
    }
}