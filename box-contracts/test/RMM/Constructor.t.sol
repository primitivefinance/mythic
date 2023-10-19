// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../../src/RMM.sol";

contract RMMConstructor is Test {
    function testFuzz_rmm_constructor_CorrectParams(
        address tokenX,
        address tokenY,
        uint256 sigma,
        uint256 strikePrice,
        uint256 tau
    ) public {
        RMM rmm = new RMM(
            ERC20(tokenX),
            ERC20(tokenY),
            sigma,
            strikePrice,
            tau
        );

        assertEq(address(rmm.tokenX()), tokenX);
        assertEq(address(rmm.tokenY()), tokenY);
        assertEq(rmm.sigma(), sigma);
        assertEq(rmm.strikePrice(), strikePrice);
        assertEq(rmm.tau(), tau);
    }
}
