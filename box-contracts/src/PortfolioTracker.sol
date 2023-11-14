pragma solidity ^0.8.14;

interface TokenLike {
    function balanceOf(address) external view returns (uint256);
}

contract PortfolioTracker {
    event LogPortfolio(uint256 tokenXBalance, uint256 tokenYBalance, uint256 blockTimestamp);
    event GhostEvent(bool ghosted);
    function logPortfolio(address tokenX, address tokenY) external {
        uint256 tokenXBalance = TokenLike(tokenX).balanceOf(msg.sender);
        uint256 tokenYBalance = TokenLike(tokenY).balanceOf(msg.sender);
        emit LogPortfolio(tokenXBalance, tokenYBalance, block.timestamp);
    }
}