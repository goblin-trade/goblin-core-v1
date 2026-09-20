use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::axis::leg::LegQuantities;

#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct HeaderFlagsV2<In: LegQuantities> {
    pub lots: In::Lots,

    #[deku(bits = 1)]
    pub flag: bool,

    #[deku(skip, cond = "!flag", default = "0")]
    pub conditional: u8,
}

/// First byte of the calldata header.
///
/// The whole struct is packed into a single byte, least-significant bit first:
/// each `bool` takes one bit and the trailing count takes whatever is left
/// (3 bits here).
#[derive(DekuRead, Default)]
#[deku(bit_order = "lsb")]
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
    use deku::DekuReader;
    use deku::no_std_io::Cursor;
    use deku::reader::Reader;

    use super::*;

    #[test]
    fn deku_layout_matches_manual_layout() {
        // withdraw_eth occupies bit 3; custom_erc20_count occupies bits 5..7.
        // Byte `0b1010_1000` therefore decodes to those two fields.
        let byte = [0b1010_1000u8];

        let mut reader = Reader::new(Cursor::new(&byte[..]));
        let decoded = HeaderFlags::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert!(decoded.withdraw_eth);
        assert_eq!(decoded.custom_erc20_count, 5);
        assert!(!decoded.read_custom_recipient);
        assert!(!decoded.withdraw_internally);
    }
}
