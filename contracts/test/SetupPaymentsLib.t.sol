// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../script/utils/SetupPaymentsLib.sol";
import "../script/utils/CoreDeploymentLib.sol";
import "../script/utils/HelloWorldDeploymentLib.sol";
import "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import "@eigenlayer/contracts/interfaces/IStrategy.sol";
import "../script/DeployEigenLayerCore.s.sol";
import "../script/HelloWorldDeployer.s.sol";
import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";
import {HelloWorldTaskManagerSetup} from "test/HelloWorldServiceManager.t.sol";
import {
    Quorum,
    StrategyParams,
    IStrategy
} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";

contract TestConstants {
    uint256 constant NUM_PAYMENTS = 8;
    uint256 constant NUM_TOKEN_EARNINGS = 1;
    uint256 constant TOKEN_EARNINGS = 100;

    address RECIPIENT = address(1);
    address EARNER = address(2);
    uint256 INDEX_TO_PROVE = 0;
}

contract SetupPaymentsLibTest is Test, TestConstants, HelloWorldTaskManagerSetup {
    using SetupPaymentsLib for *;

    IRewardsCoordinator public rewardsCoordinator;
    IStrategy public strategy;

    
    function setUp() public override virtual {
        address proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        coreConfigData =
           CoreDeploymentLib.readDeploymentConfigValues("test/mockData/config/core/", 1337); // TODO: Fix this to correct path
        coreDeployment = CoreDeploymentLib.deployContracts(proxyAdmin, coreConfigData);

        mockToken = new ERC20Mock();

        strategy = addStrategy(address(mockToken)); // Similar function to HW_SM test using strategy factory
        quorum.strategies.push(StrategyParams({strategy: strategy, multiplier: 10_000}));

        helloWorldDeployment =
            HelloWorldDeploymentLib.deployContracts(proxyAdmin, coreDeployment, quorum);
        labelContracts(coreDeployment, helloWorldDeployment);

        rewardsCoordinator = IRewardsCoordinator(coreDeployment.rewardsCoordinator);
    }


    function testSubmitPaymentRoot() public {
        address[] memory earners = new address[](8);
        for (uint256 i = 0; i < earners.length; i++) {
            earners[i] = address(1);
        }


        // SetupPaymentsLib.submitPaymentRoot(rewardsCoordinator, earners, address(strategy));
        SetupPaymentsLib.submitPaymentRoot(rewardsCoordinator, earners, address(strategy), 8, 1, 100);
    }

    function testWriteLeavesToJson() public {
        bytes32[] memory leaves = new bytes32[](2);
        leaves[0] = bytes32(uint256(1));
        leaves[1] = bytes32(uint256(2));

        bytes32[] memory tokenLeaves = new bytes32[](2);
        tokenLeaves[0] = bytes32(uint256(3));
        tokenLeaves[1] = bytes32(uint256(4));

        SetupPaymentsLib.writeLeavesToJson(leaves, tokenLeaves);

        assertTrue(vm.exists("payments.json"), "JSON file should be created");
    }

    function testParseLeavesFromJson() public {
        string memory filePath = "test_parse_payments.json";
        string memory jsonContent = '{"leaves":["0x1234"], "tokenLeaves":["0x5678"]}';
        vm.writeFile(filePath, jsonContent);

        SetupPaymentsLib.PaymentLeaves memory paymentLeaves = SetupPaymentsLib.parseLeavesFromJson(filePath);

        assertEq(paymentLeaves.leaves.length, 1, "Incorrect number of leaves");
        assertEq(paymentLeaves.tokenLeaves.length, 1, "Incorrect number of token leaves");
    }

    function testGenerateMerkleProof() public {
        SetupPaymentsLib.PaymentLeaves memory paymentLeaves = SetupPaymentsLib.parseLeavesFromJson("test/mockData/scratch/payment_leaves.json");

        bytes32[] memory leaves = paymentLeaves.leaves;
        uint256 indexToProve = 0;

        bytes32[] memory proof = new bytes32[](2);
        proof[0] = leaves[1];
        proof[1] = keccak256(abi.encodePacked(leaves[2], leaves[3]));
        
        bytes memory proofBytes1 = abi.encodePacked(proof);
        bytes memory proofBytes2 = SetupPaymentsLib.generateMerkleProof(leaves, indexToProve);

        require(keccak256(proofBytes1) == keccak256(proofBytes2), "Proofs do not match");
    }
 
     function testProcessClaim() public {
        string memory filePath = "test/mockData/scratch/payment_leaves.json";
        IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf;
        earnerLeaf.earner = EARNER;

        SetupPaymentsLib.processClaim(
            rewardsCoordinator,
            filePath,
            INDEX_TO_PROVE,
            RECIPIENT,
            earnerLeaf,
            NUM_TOKEN_EARNINGS,
            address(strategy)
        );


    }

    function testCreatePaymentSubmissions() public {
        uint256 numPayments = 5;
        uint256 amountPerPayment = 100;
        uint32 duration = 7 days;

        SetupPaymentsLib.createPaymentSubmissions(
            rewardsCoordinator,
            address(strategy),
            numPayments,
            amountPerPayment,
            duration
        );
    }
}


contract MockRewardsCoordinator is IRewardsCoordinator, TestConstants {
    function processClaim(RewardsMerkleClaim calldata claim, address recipient) external override {
        // Basic input checks
        require(claim.rootIndex < type(uint32).max, "Invalid root index");
        require(claim.earnerIndex < type(uint32).max, "Invalid earner index");
        require(claim.earnerTreeProof.length > 0, "Empty earner tree proof");
        require(claim.earnerLeaf.earner == EARNER, "Invalid earner address");
        require(recipient == RECIPIENT, "Invalid recipient address");
        require(claim.tokenIndices.length == claim.tokenTreeProofs.length, "Mismatched token proofs");
        require(claim.tokenIndices.length == claim.tokenLeaves.length, "Mismatched token leaves");
        require(recipient != address(0), "Invalid recipient address");
    }

    function createAVSRewardsSubmission(RewardsSubmission[] calldata submissions) external override {
        require(submissions.length > 0, "Empty submissions array");

        for (uint256 i = 0; i < submissions.length; i++) {
            RewardsSubmission memory submission = submissions[i];
            
            require(submission.strategiesAndMultipliers.length > 0, "Empty strategies array");
            require(submission.token != IERC20(address(0)), "Invalid token address");
            require(submission.amount > 0, "Invalid amount");
            require(submission.startTimestamp >= block.timestamp, "Invalid start timestamp");
            require(submission.duration > 0, "Invalid duration");

            uint256 totalMultiplier = 0;
            for (uint256 j = 0; j < submission.strategiesAndMultipliers.length; j++) {
                require(address(submission.strategiesAndMultipliers[j].strategy) != address(0), "Invalid strategy address");
                require(submission.strategiesAndMultipliers[j].multiplier > 0, "Invalid multiplier");
                totalMultiplier += submission.strategiesAndMultipliers[j].multiplier;
            }
            require(totalMultiplier == 10000, "Total multiplier must be 10000");
        }

        // If all checks pass, emit an event to simulate successful submission
        emit AVSRewardsSubmissionCreated(msg.sender, submissions.length);
    }

    // Event to simulate successful submission
    event AVSRewardsSubmissionCreated(address submitter, uint256 submissionCount);

    function rewardsUpdater() external view returns (address){}

    function CALCULATION_INTERVAL_SECONDS() external view returns (uint32){}

    function MAX_REWARDS_DURATION() external view returns (uint32){}

    function MAX_RETROACTIVE_LENGTH() external view returns (uint32){}

    function MAX_FUTURE_LENGTH() external view returns (uint32){}

    function GENESIS_REWARDS_TIMESTAMP() external view returns (uint32){}

    function activationDelay() external view returns (uint32){}

    function claimerFor(address earner) external view returns (address){}

    function cumulativeClaimed(address claimer, IERC20 token) external view returns (uint256){}

    function globalOperatorCommissionBips() external view returns (uint16){}

    function operatorCommissionBips(address operator, address avs) external view returns (uint16){}

    function calculateEarnerLeafHash(EarnerTreeMerkleLeaf calldata leaf) external pure returns (bytes32){}

    function calculateTokenLeafHash(TokenTreeMerkleLeaf calldata leaf) external pure returns (bytes32){
        return keccak256(abi.encodePacked(leaf.cumulativeEarnings));
    }

    function checkClaim(RewardsMerkleClaim calldata claim) external view returns (bool){}

    function currRewardsCalculationEndTimestamp() external view returns (uint32){}

    function getDistributionRootsLength() external view returns (uint256){}

    function getDistributionRootAtIndex(uint256 index) external view returns (DistributionRoot memory){}

    function getCurrentDistributionRoot() external view returns (DistributionRoot memory){}

    function getCurrentClaimableDistributionRoot() external view returns (DistributionRoot memory){}

    function getRootIndexFromHash(bytes32 rootHash) external view returns (uint32){}


    function disableRoot(uint32 rootIndex) external{}


    function setClaimerFor(address claimer) external{}

    function setActivationDelay(uint32 _activationDelay) external{}

    function setGlobalOperatorCommission(uint16 _globalCommissionBips) external{}

    function setRewardsUpdater(address _rewardsUpdater) external{}

    function setRewardsForAllSubmitter(address _submitter, bool _newValue) external{}

    function createRewardsForAllSubmission(RewardsSubmission[] calldata rewardsSubmission) external{}

    function submitRoot(bytes32 root, uint32 rewardsCalculationEndTimestamp) external{}
}


contract MockStrategy is IStrategy {

    IERC20 public token;

    constructor(IERC20 _token) {
        token = _token;
    }

    function deposit(IERC20 token, uint256 amount) external returns (uint256){}

    function withdraw(address recipient, IERC20 token, uint256 amountShares) external{}

    function sharesToUnderlying(uint256 amountShares) external returns (uint256){}

    function underlyingToShares(uint256 amountUnderlying) external returns (uint256){}

    function userUnderlying(address user) external returns (uint256){}

    function shares(address user) external view returns (uint256){}

    function sharesToUnderlyingView(uint256 amountShares) external view returns (uint256){}

    function underlyingToSharesView(uint256 amountUnderlying) external view returns (uint256){}

    function userUnderlyingView(address user) external view returns (uint256){}

    function underlyingToken() external view returns (IERC20){
        return token;
    }

    function totalShares() external view returns (uint256){}

    function explanation() external view returns (string memory){}
}

