// SPDX-License-Identifier: UNLICENSED
pragma solidity ^ 0.8.24;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Ssal {
    mapping(bytes32 => Rollup) private rollups;

    uint256 public constant MAX_SEQUENCER_COUNT = 30;

    struct Rollup {
        address owner;
        mapping(address => bool) isRegisteredSequencer;
        address[MAX_SEQUENCER_COUNT] sequencerAddresses;
        uint256 currentSequencerCount;
        uint ethBlockMargin;
    }

    event InitializeRollup(bytes32 rollupId);
    event RegisterSequencer(bytes32 rollupId, address sequencerAddr);
    event DeregisterSequencer(bytes32 rollupId, address sequencerAddr);

    function getSequencers(bytes32 rollupId) public view returns(address[] memory) {
        Rollup storage rollup = rollups[rollupId];
        address[] memory sequencerList = new address[](rollup.currentSequencerCount);
        uint256 count = 0;
        for (uint256 i = 0; i < MAX_SEQUENCER_COUNT; i++) {
            if (rollup.isRegisteredSequencer[rollup.sequencerAddresses[i]]) {
                sequencerList[count] = rollup.sequencerAddresses[i];
                count++;
            }
        }
        return sequencerList;
    }

    function initializeRollup() public {
        bytes32 rollupId = keccak256(abi.encodePacked(msg.sender, blockhash(block.number - 1)));
        
        Rollup storage rollup = rollups[rollupId];
        rollup.owner = msg.sender;
        rollup.currentSequencerCount = 0;
        
        emit InitializeRollup(rollupId);
    }

    function registerSequencer(bytes32 rollupId) public {
        Rollup storage rollup = rollups[rollupId];
        require(!rollup.isRegisteredSequencer[msg.sender], "Already registered sequencer");
        require(rollup.currentSequencerCount < MAX_SEQUENCER_COUNT, "Max sequencer count exceeded");

        rollup.isRegisteredSequencer[msg.sender] = true;
        rollup.sequencerAddresses[rollup.currentSequencerCount] = msg.sender;
        rollup.currentSequencerCount++;

        emit RegisterSequencer(rollupId, msg.sender);
    }

    function deregisterSequencer(bytes32 rollupId) public {
        Rollup storage rollup = rollups[rollupId];
        require(rollup.isRegisteredSequencer[msg.sender], "Not registered sequencer");

        rollup.isRegisteredSequencer[msg.sender] = false;

        // Remove sequencer from the array
        for (uint256 i = 0; i < rollup.currentSequencerCount; i++) {
            if (rollup.sequencerAddresses[i] == msg.sender) {
                rollup.sequencerAddresses[i] = rollup.sequencerAddresses[rollup.currentSequencerCount - 1];
                rollup.sequencerAddresses[rollup.currentSequencerCount - 1] = address(0);
                break;
            }
        }

        rollup.currentSequencerCount--;

        emit DeregisterSequencer(rollupId, msg.sender);
    }
}
