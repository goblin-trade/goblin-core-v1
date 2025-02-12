use crate::{
    native_keccak256,
    quantities::{BaseLots, QuoteLots, Ticks},
    types::Address,
};

#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MarketParams {
    pub base_token: Address,
    pub quote_token: Address,
    pub base_lot_size: BaseLots,
    pub quote_lot_size: QuoteLots,
    pub tick_size: Ticks,
    pub taker_fee_bps: u16,
    pub fee_collector: Address,
    pub base_decimals_to_ignore: u8,
    pub quote_decimals_to_ignore: u8,
}

impl MarketParams {
    pub fn keccak256(&self) -> [u8; 32] {
        let mut output = [0u8; 32];
        unsafe {
            native_keccak256(
                (self as *const Self) as *const u8,
                core::mem::size_of::<Self>(),
                output.as_mut_ptr(),
            );
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use tiny_keccak::Hasher;

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

    #[test]
    fn test_keccak() {
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
        let result = market_params.keccak256();

        let bytes = unsafe {
            std::slice::from_raw_parts(
                &market_params as *const MarketParams as *const u8,
                core::mem::size_of::<MarketParams>(),
            )
        };
        let mut hasher = tiny_keccak::Keccak::v256();
        hasher.update(bytes);
        let mut expected_hash = [0u8; 32];
        hasher.finalize(&mut expected_hash);

        assert_eq!(result, expected_hash);
    }
}
