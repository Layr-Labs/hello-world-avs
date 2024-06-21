// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface IHelloWorldServiceManager {
    // EVENTS
    event NewTaskCreated(uint32 indexed taskIndex, Task task, bytes commitment, uint64 blockNumber, uint32 rollupID, bytes32 clusterID, uint32 taskCreatedBlock);

    event TaskResponded(uint32 indexed taskIndex, bytes commitment, uint64 blockNumber, uint32 rollupID, bytes32 clusterID, uint32 taskCreatedBlock, address operator);

    // STRUCTS
    struct Task {
        bytes commitment;
        uint64 blockNumber;
        uint32 rollupID;
        bytes32 clusterID;
        uint32 taskCreatedBlock;
    }

    // FUNCTIONS
    // NOTE: this function creates new task.
    function createNewTask(
        bytes calldata _commitment,
        uint64 _blockNumber,
        uint32 _rollupID,
        bytes32 _clusterID
    ) external;

    // NOTE: this function is called by operators to respond to a task.
    function respondToTask(
        Task calldata task,
        uint32 referenceTaskIndex,
        bytes calldata signature
    ) external;
}