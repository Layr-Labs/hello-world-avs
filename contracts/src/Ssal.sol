// SPDX-License-Identifier: UNLICENSED
pragma solidity ^ 0.8.24;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Ssal {
    mapping(bytes32 => ProposerSet) private proposerSets;

    uint256 public constant MAX_SEQUENCER_COUNT = 30;
    uint256 public constant BLOCK_MARGIN = 7;

    struct ProposerSet {
        address owner; // Owner is rollup contract address
        
        mapping(address => bool) isRegisteredSequencer;
        address[MAX_SEQUENCER_COUNT] sequencerAddresses;
        uint256 currentSequencerCount;
    }

    event InitializeProposerSet(bytes32 proposerSetId, address owner);
    event RegisterSequencer(bytes32 proposerSetId, address sequencerAddress);
    event DeregisterSequencer(bytes32 proposerSetId, address sequencerAddress);

    function initializeProposerSet() public {
        bytes32 proposerSetId = keccak256(abi.encodePacked(msg.sender, blockhash(block.number - 1)));
        
        ProposerSet storage proposerSet = proposerSets[proposerSetId];
        
        require(proposerSet.owner == address(0), "Already initialized proposer set");

        proposerSet.owner = msg.sender;
        proposerSet.currentSequencerCount = 0;
        
        emit InitializeProposerSet(proposerSetId, msg.sender);
    }

    function registerSequencer(bytes32 proposerSetId) public {
        ProposerSet storage proposerSet = proposerSets[proposerSetId];

        require(!proposerSet.isRegisteredSequencer[msg.sender], "Already registered sequencer");
        require(proposerSet.currentSequencerCount < MAX_SEQUENCER_COUNT, "Max sequencer count exceeded");

        proposerSet.isRegisteredSequencer[msg.sender] = true;

        // Remove sequencer from the array
        for (uint256 i = 0; i < proposerSet.currentSequencerCount; i++) {
            if (proposerSet.sequencerAddresses[i] == address(0)) {
                proposerSet.sequencerAddresses[i] = msg.sender;
                break;
            }
        }

        proposerSet.currentSequencerCount++;

        emit RegisterSequencer(proposerSetId, msg.sender);
    }

    function deregisterSequencer(bytes32 proposerSetId) public {
        ProposerSet storage proposerSet = proposerSets[proposerSetId];

        require(proposerSet.isRegisteredSequencer[msg.sender], "Not registered sequencer");

        proposerSet.isRegisteredSequencer[msg.sender] = false;

        // Remove sequencer from the array
        for (uint256 i = 0; i < proposerSet.currentSequencerCount; i++) {
            if (proposerSet.sequencerAddresses[i] == msg.sender) {
                proposerSet.sequencerAddresses[i] = address(0);
                break;
            }
        }

        proposerSet.currentSequencerCount--;

        emit DeregisterSequencer(proposerSetId, msg.sender);
    }

    function getSequencerList(bytes32 proposerSetId) public view returns(address[MAX_SEQUENCER_COUNT] memory) {      
        return proposerSets[proposerSetId].sequencerAddresses;
    }
}
