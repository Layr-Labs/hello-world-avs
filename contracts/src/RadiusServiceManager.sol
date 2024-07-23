// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "@eigenlayer/contracts/core/DelegationManager.sol";
import "@eigenlayer-middleware/src/unaudited/ECDSAServiceManagerBase.sol";
import "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import "@openzeppelin-upgrades/contracts/utils/cryptography/ECDSAUpgradeable.sol";
import "@eigenlayer/contracts/permissions/Pausable.sol";
import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {IAVSDirectory} from "@eigenlayer-middleware/lib/eigenlayer-contracts/src/contracts/interfaces/IAVSDirectory.sol";
import "./IRadiusServiceManager.sol";

/**
 * @title Primary entrypoint for procuring services from Radius.
 */
contract RadiusServiceManager is 
    ECDSAServiceManagerBase,
    IRadiusServiceManager,
    Pausable
{
    using BytesLib for bytes;
    using ECDSAUpgradeable for bytes32;

    /* STORAGE */
    // The latest task index
    uint32 public latestTaskIndex;

    // mapping of task indices to all tasks hashes
    // when a task is created, task hash is stored here,
    // and responses need to pass the actual task,
    // which is hashed onchain and checked against this mapping
    mapping(uint32 => bytes32) public allTaskHashes;

    // mapping of task indices to hash of abi.encode(taskResponse, taskResponseMetadata)
    mapping(address => mapping(uint32 => bytes)) public allTaskResponses;

    /* MODIFIERS */
    modifier onlyOperator() {
        require(
            ECDSAStakeRegistry(stakeRegistry).operatorRegistered(msg.sender) 
            == 
            true, 
            "Operator must be the caller"
        );
        _;
    }

    constructor(
        address _avsDirectory,
        address _stakeRegistry,
        address _delegationManager
    )
        // address(0): Address of the payment coordinator contract, which handles payment distributions. (Todo: Implement this)
        ECDSAServiceManagerBase(
            _avsDirectory,
            _stakeRegistry,
            address(0), 
            _delegationManager
        )
    {}

    /* FUNCTIONS */
    // NOTE: This function creates new task, assigns it a taskId
    function createTask(
        bytes calldata _commitment,
        uint64 _blockNumber,
        bytes32 _proposerSetId,
        uint32 _fullNodeId
    ) external {
        // create a new task struct
        Task memory newTask;
        newTask.commitment = _commitment;
        newTask.blockNumber = _blockNumber;
        newTask.proposerSetId = _proposerSetId;
        newTask.fullNodeId = _fullNodeId;
        
        newTask.taskCreatedBlock = uint32(block.number);

        // Store hash of task onchain, emit event, and increase taskNumber
        allTaskHashes[latestTaskIndex] = keccak256(abi.encode(newTask));

        emit TaskCreated(latestTaskIndex, newTask);

        latestTaskIndex = latestTaskIndex + 1;
    }

    // NOTE: This function responds to existing tasks.
    function respondToTask(
        Task calldata task,
        uint32 taskIndex,
        bytes calldata signature
    ) external onlyOperator {
        require(
            operatorHasMinimumWeight(msg.sender),
            "Operator does not have match the weight requirements"
        );

        // Check that the task is valid, hasn't been responsed yet, and is being responded in time
        require(
            keccak256(abi.encode(task)) ==
                allTaskHashes[taskIndex],
            "supplied task does not match the one recorded in the contract"
        );

        require(
            allTaskResponses[msg.sender][taskIndex].length == 0,
            "Operator has already responded to the task"
        );

        // Validate the signature
        bytes32 messageHash = keccak256(task.commitment);
        bytes32 ethSignedMessageHash = messageHash.toEthSignedMessageHash();
        address signer = ethSignedMessageHash.recover(signature);
        require(signer == msg.sender, "Invalid signature");

        // Update the storage with task responses
        allTaskResponses[msg.sender][taskIndex] = signature;

        emit TaskResponded(taskIndex, msg.sender);
    }

    function updateAVSMetadata(
        string memory _metadataURI
    ) external {
        IAVSDirectory(avsDirectory).updateAVSMetadataURI(_metadataURI);
    }

    // HELPER
    function operatorHasMinimumWeight(address operator) public view returns (bool) {
        return ECDSAStakeRegistry(stakeRegistry).getOperatorWeight(operator) >= ECDSAStakeRegistry(stakeRegistry).minimumWeight();
    }
}