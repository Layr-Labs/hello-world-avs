// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.12;

// import "../src/HelloWorldServiceManager.sol" as hwsm;
// import {BLSMockAVSDeployer} from "@eigenlayer-middleware/test/utils/BLSMockAVSDeployer.sol";
// import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

// contract HelloWorldTaskManagerTest is BLSMockAVSDeployer {
//     hwsm.HelloWorldServiceManager sm;
//     hwsm.HelloWorldServiceManager smImplementation;

//     address generator =
//         address(uint160(uint256(keccak256(abi.encodePacked("generator")))));

//     function setUp() public {
//         _setUpBLSMockAVSDeployer();

//         tmImplementation = new HelloWorldTaskManager(
//             incsqsm.IRegistryCoordinator(address(registryCoordinator)),
//             TASK_RESPONSE_WINDOW_BLOCK
//         );

//         // Third, upgrade the proxy contracts to use the correct implementation contracts and initialize them.
//         tm = HelloWorldTaskManager(
//             address(
//                 new TransparentUpgradeableProxy(
//                     address(tmImplementation),
//                     address(proxyAdmin),
//                     abi.encodeWithSelector(
//                         tm.initialize.selector,
//                         pauserRegistry,
//                         registryCoordinatorOwner,
//                         aggregator,
//                         generator
//                     )
//                 )
//             )
//         );
//     }

//     function testCreateNewTask() public {
//         bytes memory quorumNumbers = new bytes(0);
//         cheats.prank(generator, generator);
//         tm.createNewTask(2, 100, quorumNumbers);
//         assertEq(tm.latestTaskNum(), 1);
//     }
// }
