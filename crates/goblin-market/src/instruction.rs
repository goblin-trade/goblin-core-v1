pub enum GoblinInstruction {
    /// Place a limit order on the book. The order can cross if the supplied order type is Limit
    PlaceLimitOrder = 2,

    /// Place a limit order on the book using only deposited funds.
    PlaceLimitOrderWithFreeFunds = 3,
}
