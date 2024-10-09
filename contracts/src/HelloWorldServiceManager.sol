// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {ECDSAServiceManagerBase} from
    "@eigenlayer-middleware/src/unaudited/ECDSAServiceManagerBase.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {IServiceManager} from "@eigenlayer-middleware/src/interfaces/IServiceManager.sol";
import {ECDSAUpgradeable} from
    "@openzeppelin-upgrades/contracts/utils/cryptography/ECDSAUpgradeable.sol";
import {IERC1271Upgradeable} from
    "@openzeppelin-upgrades/contracts/interfaces/IERC1271Upgradeable.sol";
import {IHelloWorldServiceManager} from "./IHelloWorldServiceManager.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title Primary entrypoint for procuring services from HelloWorld.
 * @author Eigen Labs, Inc.
 */
contract HelloWorldServiceManager is ECDSAServiceManagerBase, IHelloWorldServiceManager {
    using ECDSAUpgradeable for bytes32;

    uint32 public latestTaskNum;

    address public aggregator;

    // mapping of task indices to all tasks hashes
    // when a task is created, task hash is stored here,
    // and responses need to pass the actual task,
    // which is hashed onchain and checked against this mapping
    mapping(uint32 => bytes32) public allTaskHashes;

    // mapping of task indices to hash of abi.encode(taskResponse, taskResponseMetadata)
    mapping(uint32 => bytes) public allTaskResponses;

    modifier onlyAggregator() {
        if (msg.sender != aggregator) {
            revert("Aggregator must be the caller");
        }
        _;
    }

    constructor(
        address _avsDirectory,
        address _stakeRegistry,
        address _delegationManager,
        address _aggregator
    )
        ECDSAServiceManagerBase(
            _avsDirectory,
            _stakeRegistry,
            address(0), // hello-world doesn't need to deal with payments
            _delegationManager
        )
    {}

    function initialize(address _aggregator) public initializer {
        _updateAggregator(_aggregator);
    }

    /* FUNCTIONS */
    // NOTE: this function creates new task, assigns it a taskId
    function createNewTask(string memory name) external returns (Task memory) {
        // create a new task struct
        Task memory newTask;
        newTask.name = name;
        newTask.taskCreatedBlock = uint32(block.number);

        // store hash of task onchain, emit event, and increase taskNum
        allTaskHashes[latestTaskNum] = keccak256(abi.encode(newTask));
        emit NewTaskCreated(latestTaskNum, newTask);
        latestTaskNum = latestTaskNum + 1;

        return newTask;
    }

    function respondToTask(
        Task calldata task,
        uint32 referenceTaskIndex,
        bytes memory aggregateSignatureData
    ) external onlyAggregator {
        // check that the task is valid, hasn't been responsed yet, and is being responded in time
        require(
            keccak256(abi.encode(task)) == allTaskHashes[referenceTaskIndex],
            "supplied task does not match the one recorded in the contract"
        );

        // The message that was signed
        bytes32 messageHash = keccak256(abi.encodePacked("Hello, ", task.name));

        (address[] memory operators, bytes[] memory signatures, uint32 referenceBlock) =
            abi.decode(aggregateSignatureData, (address[], bytes[], uint32));

        // check if all signers are registered operators
        // @note is this necessary? _checkSignatures requires that the total signer weight meets our threshold
        // only registered operators can contribute to the weight.
        for (uint256 i = 0; i < operators.length; i++) {
            require(_isOperator(operators[i]), "Invalid signer: not registered as operator");
        }

        // check if signatures are valid
        bytes4 magicValue = IERC1271Upgradeable.isValidSignature.selector;
        bytes32 ethSignedMessageHash = messageHash.toEthSignedMessageHash();
        if (
            !(
                magicValue
                    == ECDSAStakeRegistry(stakeRegistry).isValidSignature(
                        ethSignedMessageHash, aggregateSignatureData
                    )
            )
        ) {
            revert("Invalid signature");
        }

        // updating the storage with task response
        allTaskResponses[referenceTaskIndex] = aggregateSignatureData;

        // emitting event
        emit TaskResponded(referenceTaskIndex, task, msg.sender);
    }

    function updateAggregator(address _newAggregator) external onlyOwner {
        _updateAggregator(_newAggregator);
    }

    function _isOperator(address _operator) internal view returns (bool) {
        return ECDSAStakeRegistry(stakeRegistry).operatorRegistered(_operator);
    }

    function _updateAggregator(address _newAggregator) internal {
        address oldAggregator = aggregator;
        aggregator = _newAggregator;
        emit AggregatorUpdated(oldAggregator, _newAggregator);
    }
}
