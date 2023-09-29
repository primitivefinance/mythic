// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IG3M {
    /// @notice Emitted when liquidity is added to the pool.
    event AddLiquidity(
        address indexed sender,
        uint256 liquidity,
        uint256 amountX,
        uint256 amountY
    );

    /// @notice Emitted when liquidity is removed from the pool.
    event RemoveLiquidity(
        address indexed sender,
        uint256 liquidity,
        uint256 amountX,
        uint256 amountY
    );

    /// @notice Emitted when a swap occurs.
    event Swap(
        address indexed sender,
        bool swapDirection,
        uint256 input,
        uint256 output
    );

    /// @notice Emitted when the weight of token X is updated.
    event UpdateWeightX(uint256 oldWeightX, uint256 newWeightX);

    event SetTargetWeightX(
        uint256 newTargetWeightX,
        uint256 newWeightXUpdateEnd,
        uint256 newWeightXUpdatePerSecond
    );

    /**
     * @notice Initializes the pool before any liquidity can be added. This
     * function can only be called once. Note that the ratio between `amountX`
     * and `amountY` will determine the initial spot price of the pool.
     * @dev The reason why this function exists is that we need some initial
     * values before we can actually compute anything. Hence we calculate the
     * initial quantity of liquidity by multiplying the invariant by 2.
     * Note that this function could be merged with `addLiquidity` but this
     * requires a little bit of refactoring.
     * @param amountX Amount of token X to deposit
     * @param amountY Amount of token Y to deposit
     * @return liquidity Amount of liquidity received by the sender (minus the
     * initial burnt liquidity)
     */
    function initPool(
        uint256 amountX,
        uint256 amountY
    ) external returns (uint256 liquidity);

    /**
     * @notice Adds `liquidity` units of liquidity to the pool.
     * @param liquidity Amount of liquidity to add
     * @return amountX Amount of token X used to add liquidity
     * @return amountY Amount of token Y used to add liquidity
     */
    function addLiquidity(uint256 liquidity)
        external
        returns (uint256 amountX, uint256 amountY);

    /**
     * Removes `liquidity` units of liquidity from the pool.
     * @param liquidity Amount of liquidity to remove
     * @return amountX Amount of token X received
     * @return amountY Amount of token Y received
     */
    function removeLiquidity(uint256 liquidity)
        external
        returns (uint256 amountX, uint256 amountY);

    /**
     * @notice Swaps exactly `amountIn` tokens for a maximum of `amountOut`.
     * @param swapDirection True to swap token X for Y, false otherwise
     * @param amountIn Exact amount sent by the sender
     * @return amountOut Amount received by the user
     */
    function swapAmountIn(
        bool swapDirection,
        uint256 amountIn
    ) external returns (uint256 amountOut);

    /**
     * @notice Swaps a minimum of `amountIn` tokens for exactly `amountOut`.
     * @param swapDirection True to swap token X for Y, false otherwise
     * @param amountOut Exact amount received by the user
     * @return amountIn Amount sent by the sender
     */
    function swapAmountOut(
        bool swapDirection,
        uint256 amountOut
    ) external returns (uint256 amountIn);

    /**
     * @notice Updates the weight of token X.
     * @param newTargetWeightX New weight of token X expressed in WAD
     */
    function updateWeightX(
        uint256 newTargetWeightX,
        uint256 newWeightXUpdateEnd,
        uint256 newWeightXUpdatePerSecond
    ) external;

    /// @notice Computes the spot price of token X in terms of token Y.
    function getSpotPrice() external view returns (uint256);

    /// @notice Address of token X.
    function tokenX() external view returns (address);

    /// @notice Address of token Y.
    function tokenY() external view returns (address);

    /// @notice Reserve of token X, stored in WAD.
    function reserveX() external view returns (uint256);

    /// @notice Reserve of token Y, stored in WAD.
    function reserveY() external view returns (uint256);

    /// @notice Total units of liquidity in the pool.
    function totalLiquidity() external view returns (uint256);

    /// @notice Units of liquidity owned by `account`.
    function balanceOf(address account) external view returns (uint256);

    /// @notice Weight of token X, expressed in WAD.
    function weightX() external view returns (uint256);

    /// @notice Weight of token Y, expressed in WAD.
    function weightY() external view returns (uint256);
}
