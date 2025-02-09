use crate::quantities::{BaseLots, QuoteLots, Ticks};

#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MarketParams {
    pub base_token: [u8; 20],
    pub quote_token: [u8; 20],
    pub base_lot_size: BaseLots,
    pub quote_lot_size: QuoteLots,
    pub tick_size: Ticks,
    pub taker_fee_bps: u16,
    pub fee_collector: [u8; 20],
    pub base_decimals_to_ignore: u8,
    pub quote_decimals_to_ignore: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_params_serialization() {
        let market_params = MarketParams {
            base_token: [0u8; 20],
            quote_token: [1u8; 20],
            base_lot_size: BaseLots(5),
            quote_lot_size: QuoteLots(2),
            tick_size: Ticks(1),
            taker_fee_bps: 2,
            fee_collector: [3u8; 20],
            base_decimals_to_ignore: 6,
            quote_decimals_to_ignore: 6,
        };

        // Serialize the struct into bytes
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &market_params as *const MarketParams as *const u8,
                core::mem::size_of::<MarketParams>(),
            )
        };
        println!("Serialized bytes: {:?}", bytes);

        let hex_string: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        println!("Serialized bytes (hex): {}", hex_string);

        // Deserialize as zero-copy
        let deserialized: &MarketParams = unsafe { &*(bytes.as_ptr() as *const MarketParams) };
        println!("Deserialized MarketParams: {:?}", deserialized);

        // Ensure correctness
        assert_eq!(market_params, *deserialized);
    }
}
