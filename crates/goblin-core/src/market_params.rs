#[repr(C, packed)]
pub struct MarketParams {
    pub base_token: [u8; 20],
    pub quote_token: [u8; 20],
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
}
