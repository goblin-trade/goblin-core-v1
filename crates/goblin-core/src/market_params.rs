#[repr(C, packed)]
#[derive(Debug, PartialEq)]
pub struct MarketParams {
    pub base_token: [u8; 20],
    pub quote_token: [u8; 20],
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_params_serialization() {
        let market_params = MarketParams {
            base_token: [0u8; 20],
            quote_token: [1u8; 20],
            base_lot_size: 1,
            quote_lot_size: 2,
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
