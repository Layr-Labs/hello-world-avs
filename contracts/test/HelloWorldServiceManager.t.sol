// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {HelloWorldServiceManager} from "../src/HelloWorldServiceManager.sol";
import {MockAVSDeployer} from "@eigenlayer-middleware/test/utils/MockAVSDeployer.sol";
import {Vm} from "forge-std/Vm.sol";
import {console2} from "forge-std/Test.sol";

contract HelloWorldTaskManagerSetup is MockAVSDeployer {
    struct Operator {
        Vm.Wallet key;
        Vm.Wallet signingKey;
    }

    struct Generator {
        Vm.Wallet key;
    }

    Operator[] internal operators;
    Generator internal generator;

    function setUp() public {
        operators.push(
            Operator({
                key: vm.createWallet("operator"),
                signingKey: vm.createWallet("operator_signing_wallet")
            })
        );

        generator = Generator({key: vm.createWallet("generator_wallet")});
    }
}

contract HelloWorldServiceManagerTest is HelloWorldTaskManagerSetup {
    function testTrue() public {
        assertTrue(true);
    }
}
