// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "src/DFMM.sol";
import "src/strategies/G3M/G3M.sol";
import "src/strategies/LogNormal/LogNormal.sol";

contract DeploymentScript is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYMENT_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        DFMM dfmm = new DFMM();
        G3M g3m = new G3M(address(dfmm));
        LogNormal logNormal = new LogNormal(address(dfmm));

        console.log("DFMM:", address(dfmm));
        console.log("G3M:", address(g3m));
        console.log("LogNormal:", address(logNormal));

        vm.stopBroadcast();
    }
}
