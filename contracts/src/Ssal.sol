// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Ssal {
    mapping(bytes32 => Cluster) clusters;
    struct Cluster {
        address owner;
        address rollup;
        uint ethBlockMargin;
        address[30] sequencers;
    }

    event InitializeClusterEvent(
        bytes32 clusterID
    );

    function getSequencers(bytes32 clusterID) public view returns (address[30] memory){        
        return clusters[clusterID].sequencers;
    }

    function initializeCluster(address _sequencer, address _rollup) public {
        bytes32 clusterID = keccak256(abi.encodePacked(_rollup, blockhash(block.number)));
        clusters[clusterID].owner = msg.sender;
        clusters[clusterID].rollup = _rollup;
        clusters[clusterID].sequencers[0] = _sequencer;
        
        emit InitializeClusterEvent(clusterID);
    }

    function registerSequencer(bytes32 clusterID, address _sequencer) public {
        uint index = 30;
        for (uint i = 0; i < 30; i++) {
            if(clusters[clusterID].sequencers[i] == _sequencer) revert();
            if(clusters[clusterID].sequencers[i] == address(0)) {
                index = i;
                break;
            }
        }
        if (index == 30) revert();
        else clusters[clusterID].sequencers[index] = _sequencer;
    }

    function deregisterSequencer(bytes32 clusterID, address _sequencer) public {
        for (uint i = 0; i < 30; i++) {
            if(clusters[clusterID].sequencers[i] == _sequencer)
                clusters[clusterID].sequencers[i] = address(0);
        }
    }
    // function registerBlockCommitment(string memory rollupId, uint64 blockHeight, string memory commitment) public {
        
    //     emit BlockCommitmentEvent(rollupId, blockHeight, commitment);
    // }
}