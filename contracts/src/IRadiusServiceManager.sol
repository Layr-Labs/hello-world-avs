// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface IRadiusServiceManager {
    // EVENTS
    event TaskCreated(uint32 indexed taskIndex, Task task);
    event TaskResponded(uint32 indexed taskIndex, address operator);

    // STRUCTS
    struct Task {
        bytes commitment;
        uint64 blockNumber;
        bytes32 proposerSetId;
        uint32 fullNodeId;
        uint32 taskCreatedBlock;
    }

    // FUNCTIONS
    // NOTE: this function creates new task.
    function createTask(
        bytes calldata _commitment,
        uint64 _blockNumber,
        bytes32 _proposerSetId,
        uint32 _fullNodeId
    ) external;

    // NOTE: this function is called by operators to respond to a task.
    function respondToTask(
        Task calldata task,
        uint32 referenceTaskIndex,
        bytes calldata signature
    ) external;

    function updateAVSMetadata(
        string memory _metadataURI
    ) external;
}