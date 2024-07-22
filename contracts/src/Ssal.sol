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
        
        mapping(address => bool) isRegisteredProposer;
        address[MAX_SEQUENCER_COUNT] proposerAddresses;
        uint256 currentProposerCount;        
    }

    event InitializeProposerSet(bytes32 proposerSetId, address owner);
    event RegisterProposer(bytes32 proposerSetId, address proposerAddress);
    event DeregisterProposer(bytes32 proposerSetId, address proposerAddress);

    function initializeProposerSet() public {
        bytes32 proposerSetId = keccak256(abi.encodePacked(msg.sender, blockhash(block.number - 1)));
        
        ProposerSet storage proposerSet = proposerSets[proposerSetId];
        proposerSet.owner = msg.sender;
        proposerSet.currentProposerCount = 0;
        
        emit InitializeProposerSet(proposerSetId, msg.sender);
    }

    function registerProposer(bytes32 proposerSetId) public {
        ProposerSet storage proposerSet = proposerSets[proposerSetId];

        require(!proposerSet.isRegisteredProposer[msg.sender], "Already registered proposer");
        require(proposerSet.currentProposerCount < MAX_SEQUENCER_COUNT, "Max proposer count exceeded");

        proposerSet.isRegisteredProposer[msg.sender] = true;

        // Remove proposer from the array
        for (uint256 i = 0; i < proposerSet.currentProposerCount; i++) {
            if (proposerSet.proposerAddresses[i] == address(0)) {
                proposerSet.proposerAddresses[i] = msg.sender;
                break;
            }
        }

        proposerSet.currentProposerCount++;

        emit RegisterProposer(proposerSetId, msg.sender);
    }

    function deregisterProposer(bytes32 proposerSetId) public {
        ProposerSet storage proposerSet = proposerSets[proposerSetId];

        require(proposerSet.isRegisteredProposer[msg.sender], "Not registered proposer");

        proposerSet.isRegisteredProposer[msg.sender] = false;

        // Remove proposer from the array
        for (uint256 i = 0; i < proposerSet.currentProposerCount; i++) {
            if (proposerSet.proposerAddresses[i] == msg.sender) {
                proposerSet.proposerAddresses[i] = address(0);
                break;
            }
        }

        proposerSet.currentProposerCount--;

        emit DeregisterProposer(proposerSetId, msg.sender);
    }

    function getProposerList(bytes32 proposerSetId) public view returns(address[] memory) {
        ProposerSet storage proposerSet = proposerSets[proposerSetId];
      
        return proposerSet.proposerAddresses;
    }
}
