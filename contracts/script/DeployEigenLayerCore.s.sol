// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {console2} from "forge-std/Test.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

contract LocalDeploy {
    function run() public {
        console2.log("run");
    }
}
