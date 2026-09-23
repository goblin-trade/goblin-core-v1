mod impl_deku_reader;
#[cfg(feature = "encode")]
mod impl_deku_writer;

use crate::{
    axis::{market::Market, token::Token},
    axis_helpers::{MarketSpec, TokenPair},
    types::{SameTriple, SameTuple, StoreReader},
};

pub type MarketCountsInner = SameTriple<SameTriple<u8, Token>, Token>;
pub type MarketCountsV2 = SameTuple<MarketCountsInner, Market>;

impl MarketCountsV2 {
    pub fn get_count<MS: MarketSpec>(&self) -> u8 {
        let market_counts = MS::Market::get_leg(self);
        let base_counts = <MS::Pair as TokenPair>::Base::get_leg(market_counts);

        <MS::Pair as TokenPair>::Quote::get(base_counts)
    }
}

#[cfg(test)]
mod tests {
    use deku::DekuReader;
    use deku::no_std_io::Cursor;
    use deku::reader::Reader;

    use super::MarketCountsV2;

    fn decode(bytes: &[u8], process_dynamic_markets: bool) -> MarketCountsV2 {
        let mut reader = Reader::new(Cursor::new(bytes));
        MarketCountsV2::from_reader_with_ctx(&mut reader, process_dynamic_markets).unwrap()
    }

    // Layout is `(market, base token, quote token)`, each inner `Triple` keyed
    // `ETH = 0`, `HardcodedERC20 = 1`, `CustomERC20 = 2`.
    #[test]
    fn hardcoded_counts_follow_legal_order() {
        // byte_0 low nibble  -> (Hardcoded, ETH, HardcodedERC20)
        // byte_0 high nibble -> (Hardcoded, HardcodedERC20, ETH)
        // byte_1 low nibble  -> (Hardcoded, HardcodedERC20, HardcodedERC20)
        let counts = decode(&[0x21, 0x03], false);
        let hardcoded = counts.0;

        assert_eq!(hardcoded.0.1, 1);
        assert_eq!(hardcoded.1.0, 2);
        assert_eq!(hardcoded.1.1, 3);
        // Illegal combinations are absent from the wire and stay zeroed.
        assert_eq!(hardcoded.0.0, 0);
        assert_eq!(hardcoded.2.2, 0);
    }

    #[test]
    fn dynamic_counts_follow_legal_order() {
        // Dynamic nibbles, in order:
        //   (ETH,H)=0 (ETH,C)=1 (H,ETH)=2 (H,H)=3
        //   (H,C)=4   (C,ETH)=5 (C,H)=6   (C,C)=7
        let counts = decode(&[0x00, 0x00, 0x10, 0x32, 0x54, 0x76], true);
        let dynamic = counts.1;

        assert_eq!(dynamic.0.1, 0);
        assert_eq!(dynamic.0.2, 1);
        assert_eq!(dynamic.1.0, 2);
        assert_eq!(dynamic.1.1, 3);
        assert_eq!(dynamic.1.2, 4);
        assert_eq!(dynamic.2.0, 5);
        assert_eq!(dynamic.2.1, 6);
        assert_eq!(dynamic.2.2, 7);
        // ETH/ETH is illegal, so it stays zeroed even with dynamic markets on.
        assert_eq!(dynamic.0.0, 0);
    }

    #[test]
    fn dynamic_counts_stay_zero_when_disabled() {
        let counts = decode(&[0x00, 0x00], false);

        assert_eq!(counts.1.2.2, 0);
    }
}
