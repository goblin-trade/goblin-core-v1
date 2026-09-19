use core::marker::PhantomData;

use deku::{DekuRead, DekuSize, DekuWrite};
use goblin_macros::fixed_codec;

#[derive(DekuRead, DekuWrite, DekuSize)]
pub struct HeaderFlagsV2<T> {
    #[deku(bits = 1)]
    pub flag: bool,

    #[deku(skip, cond = "!flag", default = "0")]
    pub conditional: u8,

    #[deku(skip)]
    pub marker: PhantomData<T>,
}

/// First byte of the calldata header.
///
/// The whole struct is packed into a single byte, least-significant bit first:
/// each `bool` takes one bit and the trailing count takes whatever is left
/// (3 bits here).
#[fixed_codec(bits = 8)]
#[derive(DekuRead, Default)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct HeaderFlags {
    /// Whether to read custom recipient address from payload
    #[deku(bits = "1")]
    pub read_custom_recipient: bool,

    /// Whether to read msg.value from hostio
    #[deku(bits = "1")]
    pub read_msg_value: bool,

    /// Whether to process dynamic markets
    #[deku(bits = "1")]
    pub process_dynamic_markets: bool,

    /// Whether to read ETH withdraw amount from args and withdraw ETH
    #[deku(bits = "1")]
    pub withdraw_eth: bool,

    /// Whether to credit tokens to ERC20Store or EthStore, or to actually transfer out tokens
    #[deku(bits = "1")]
    pub withdraw_internally: bool,

    /// Number of custom ERC20 tokens to read
    #[deku(bits = "3")]
    pub custom_erc20_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_processor::{ArgsReader, ArgsWriter, FixedCodec};

    #[test]
    fn round_trip_matches_manual_layout() {
        let flags = HeaderFlags {
            read_custom_recipient: false,
            read_msg_value: false,
            process_dynamic_markets: false,
            withdraw_eth: true,
            withdraw_internally: false,
            custom_erc20_count: 5,
        };

        let mut buf = [0u8; HeaderFlags::ENCODED_SIZE];
        let mut writer = ArgsWriter::new(&mut buf);
        flags.raw_fixed_encode(&mut writer);

        // withdraw_eth occupies bit 3; custom_erc20_count occupies bits 5..7.
        assert_eq!(buf[0], 0b1010_1000);

        let reader = ArgsReader::from_slice(&buf);
        let decoded = HeaderFlags::try_fixed_decode(&reader).unwrap();
        assert!(decoded.withdraw_eth);
        assert_eq!(decoded.custom_erc20_count, 5);
        assert!(!decoded.read_custom_recipient);
        assert!(!decoded.withdraw_internally);
    }
}
