// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from
    "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {stdJson} from "forge-std/StdJson.sol";

library HelloWorldDeploymentLib {
    using stdJson for *;

    bytes32 internal constant IMPLEMENTATION_SLOT =
        0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;
    bytes32 internal constant ADMIN_SLOT =
        0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103;

    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    function deployProxyAdmin() external returns (address) {
        return address(new ProxyAdmin());
    }

    function upgrade(address proxy, address impl) external {
        ProxyAdmin admin = getProxyAdmin(proxy);
        admin.upgrade(TransparentUpgradeableProxy(payable(proxy)), impl);
    }

    struct DeploymentData {
        address helloWorldServiceManager;
        address stakeRegistry;
        address delegationManager;
        address avsDirectory;
        address wethStrategy;
    }

    function readDeploymentJson(uint256 chainId) external returns (DeploymentData memory) {
        string memory directoryPath = "deployments/";
        string memory fileName = string.concat(directoryPath, vm.toString(chainId), ".json");

        require(vm.exists(fileName), "Deployment file does not exist");

        string memory json = vm.readFile(fileName);

        DeploymentData memory data;
        data.helloWorldServiceManager = json.readAddress(".contracts.helloWorldServiceManager");
        data.stakeRegistry = json.readAddress(".contracts.stakeRegistry");
        data.delegationManager = json.readAddress(".contracts.delegationManager");
        data.avsDirectory = json.readAddress(".contracts.avsDirectory");
        data.wethStrategy = json.readAddress(".contracts.wethStrategy");

        return data;
    }

    function writeDeploymentJson(
        address helloWorldServiceManager,
        address stakeRegistry,
        address delegationManager,
        address avsDirectory,
        address wethStrategy
    ) external {
        address proxyAdmin = address(getProxyAdmin(helloWorldServiceManager));
        address helloWorldServiceManagerImpl = getImplementation(helloWorldServiceManager);
        address stakeRegistryImpl = getImplementation(stakeRegistry);
        string memory deploymentData = string.concat(
            '{"lastUpdate":{"timestamp":"',
            vm.toString(block.timestamp),
            '","block_number":"',
            vm.toString(block.number),
            '"},"contracts":{"proxyAdmin":"',
            vm.toString(proxyAdmin),
            '","helloWorldServiceManager":"',
            vm.toString(helloWorldServiceManager),
            '","helloWorldServiceManagerImpl":"',
            vm.toString(helloWorldServiceManagerImpl),
            '","stakeRegistry":"',
            vm.toString(stakeRegistry),
            '","stakeRegistryImpl":"',
            vm.toString(stakeRegistryImpl),
            '","delegationManager":"',
            vm.toString(delegationManager),
            '","avsDirectory":"',
            vm.toString(avsDirectory),
            '","wethStrategy":"',
            vm.toString(wethStrategy),
            '"}}'
        );
        string memory directoryPath = "deployments/";
        string memory fileName = string.concat(directoryPath, vm.toString(block.chainid), ".json");
        if (!vm.exists(directoryPath)) {
            vm.createDir(directoryPath, true);
        }

        vm.writeFile(fileName, deploymentData);
        console2.log("Deployment artifacts written to:", fileName);
    }

    function setUpEmptyProxy(address admin) internal returns (address) {
        address emptyContract = address(new EmptyContract());
        return address(new TransparentUpgradeableProxy(emptyContract, admin, ""));
    }

    function upgradeAndCall(address proxy, address impl, bytes memory initData) external {
        ProxyAdmin admin = getProxyAdmin(proxy);
        admin.upgradeAndCall(TransparentUpgradeableProxy(payable(proxy)), impl, initData);
    }

    function getImplementation(address proxy) internal view returns (address) {
        bytes32 value = vm.load(proxy, IMPLEMENTATION_SLOT);
        return address(uint160(uint256(value)));
    }

    function getProxyAdmin(address proxy) internal view returns (ProxyAdmin) {
        bytes32 value = vm.load(proxy, ADMIN_SLOT);
        return ProxyAdmin(address(uint160(uint256(value))));
    }
}
