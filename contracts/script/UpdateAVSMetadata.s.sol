// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {IHelloWorldServiceManager} from "../src/IHelloWorldServiceManager.sol";

contract UpdateAVSMetadata is Script {
    function run() public {
        IHelloWorldServiceManager serviceManager = IHelloWorldServiceManager(0xc7AE524F4a1853012b7711a91E543668FF20Bc0D);

        vm.startBroadcast();

        string memory metadata = "https://raw.githubusercontent.com/radiusxyz/depository/master/metadata.json";
        serviceManager.updateAVSMetadata(metadata);

        console.log("Metadata:", metadata);

        vm.stopBroadcast();
    }
}