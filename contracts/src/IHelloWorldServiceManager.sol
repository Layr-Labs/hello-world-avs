// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface IHelloWorldServiceManager {
    event NewTaskCreated(uint32 indexed taskIndex, Task task);

    event TaskResponded(uint32 indexed taskIndex, Task task, address operator);

    event AggregatorUpdated(address oldAggregator, address newAggregator);

    struct Task {
        string name;
        uint32 taskCreatedBlock;
    }

    function latestTaskNum() external view returns (uint32);

    function aggregator() external view returns (address);

    function allTaskHashes(uint32 taskIndex) external view returns (bytes32);

    function allTaskResponses(uint32 taskIndex) external view returns (bytes memory);

    function createNewTask(string memory name) external returns (Task memory);

    function respondToTask(
        Task calldata task,
        uint32 referenceTaskIndex,
        bytes calldata signature
    ) external;

    function updateAggregator(address _newAggregator) external;

}
