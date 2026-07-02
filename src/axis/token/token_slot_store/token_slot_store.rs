pub trait TokenSlotStore {
    /// Decimals stored in `Store`
    /// Decimals are stored as u8 for ERC20 tokens but not for ETH
    type StoredDecimals: Clone + Copy;

    /// Padding to pad `Store` to 32 bytes
    /// ERC20 store has less padding to accomodate `decimals: u8`
    type StoredPadding: Clone + Copy;
}
