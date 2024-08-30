// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {HelloWorldServiceManager} from "../src/HelloWorldServiceManager.sol";
import {MockAVSDeployer} from "@eigenlayer-middleware/test/utils/MockAVSDeployer.sol";
import {TransparentUpgradeableProxy} from
    "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Vm} from "forge-std/Vm.sol";

contract HelloWorldTaskManagerTest is MockAVSDeployer {
    struct Operator {
        Vm.Wallet key;
        Vm.Wallet signingKey;
    }

    struct Generator {
        Vm.Wallet key;
    }

    Operator internal operator;
    Generator internal generator;

    function setUp() public {
        operator = Operator({
            key: vm.createWallet("operator_wallet"),
            signingKey: vm.createWallet("operator_signing_wallet")
        });

        generator = Generator({key: vm.createWallet("generator_wallet")});
    }

    function testTrue() public {
        vm.skip(true);
    }
}
