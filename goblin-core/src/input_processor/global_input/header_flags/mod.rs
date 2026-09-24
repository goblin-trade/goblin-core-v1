mod impl_goblin_read;

#[cfg(feature = "encode")]
mod impl_goblin_write;

/// First byte of the calldata header.
///
/// The whole struct is packed into a single byte, least-significant bit first:
/// each `bool` takes one bit and the trailing count takes whatever is left
/// (3 bits here). The byte is decoded as a lane in the
/// [`bit_lane`](crate::input_processor::bit_lane) helpers.
#[derive(Default, Clone, Copy)]
pub struct HeaderFlags {
    /// Whether to read custom recipient address from payload
    pub read_custom_recipient: bool,

    /// Whether to read msg.value from hostio
    pub read_msg_value: bool,

    /// Whether to process dynamic markets
    pub process_dynamic_markets: bool,

    /// Whether to read ETH withdraw amount from args and withdraw ETH
    pub withdraw_eth: bool,

    /// Whether to credit tokens to ERC20Store or EthStore, or to actually transfer out tokens
    pub withdraw_internally: bool,

    /// Number of custom ERC20 tokens to read
    pub custom_erc20_count: usize,
}

#[cfg(all(test, feature = "encode"))]
mod tests {
    use crate::codec::{GoblinRead, GoblinWrite, Reader, Writer};

    use super::HeaderFlags;

    #[test]
    fn round_trip() {
        let flags = HeaderFlags {
            read_custom_recipient: true,
            read_msg_value: true,
            process_dynamic_markets: false,
            withdraw_eth: true,
            withdraw_internally: false,
            custom_erc20_count: 5,
        };

        let mut buf = [0u8; 1];
        let mut writer = Writer::new(&mut buf);
        flags.to_writer(&mut writer, ()).unwrap();

        let mut reader = Reader::new(&buf);
        let decoded = HeaderFlags::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert!(decoded.read_custom_recipient);
        assert!(decoded.read_msg_value);
        assert!(!decoded.process_dynamic_markets);
        assert!(decoded.withdraw_eth);
        assert!(!decoded.withdraw_internally);
        assert_eq!(decoded.custom_erc20_count, 5);
    }
}
