// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.12;

// import "../src/HelloWorldServiceManager.sol" as hwsm;
// import {HelloWorldTaskManager} from "../src/HelloWorldTaskManager.sol";
// import {MockAVSDeployer} from "@eigenlayer-middleware/test/utils/MockAVSDeployer.sol";
// import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

// contract HelloWorldTaskManagerTest is MockAVSDeployer {
//     incsqsm.HelloWorldServiceManager sm;
//     incsqsm.HelloWorldServiceManager smImplementation;
//     HelloWorldTaskManager tm;
//     HelloWorldTaskManager tmImplementation;

//     address operator =
//         address(uint160(uint256(keccak256(abi.encodePacked("operator")))));
//     address generator =
//         address(uint160(uint256(keccak256(abi.encodePacked("generator")))));

//     function setUp() public {
//         _setUpBLSMockAVSDeployer();

//         tmImplementation = new HelloWorldTaskManager(
//             incsqsm.IRegistryCoordinator(address(registryCoordinator))
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
//                         registryCoordinatorOwner
//                     )
//                 )
//             )
//         );
//     }

//     function testCreateNewTask() public {
//         cheats.prank(generator, generator);
//         tm.createNewTask("world");
//         assertEq(tm.latestTaskNum(), 1);
//     }
// }
