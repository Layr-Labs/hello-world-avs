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

    event registerSequencerEvent(
        string ip
    );

    function getSequencers(bytes32 clusterID) public view returns (address[30] memory){        
        return clusters[clusterID].sequencers;
    }

    function initializeCluster() public {
        bytes32 clusterID = keccak256(abi.encodePacked(msg.sender, blockhash(block.number-1)));
        clusters[clusterID].owner = msg.sender;
        clusters[clusterID].rollup = msg.sender;
        
        emit InitializeClusterEvent(clusterID);
    }

    function registerSequencer(bytes32 clusterID, address _sequencer, string memory ip) public {
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

        emit registerSequencerEvent(ip);
    }

    function deregisterSequencer(bytes32 clusterID, address _sequencer) public {
        for (uint i = 0; i < 30; i++) {
            if(clusters[clusterID].sequencers[i] == _sequencer)
                clusters[clusterID].sequencers[i] = address(0);
        }
    }
}