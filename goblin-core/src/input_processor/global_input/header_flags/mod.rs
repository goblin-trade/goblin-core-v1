#[cfg(feature = "encode")]
use deku::DekuWriter;
#[cfg(feature = "encode")]
use deku::no_std_io::Write;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;
#[cfg(feature = "encode")]
use deku::writer::Writer;
use deku::{DekuError, DekuReader};

#[cfg(feature = "encode")]
use crate::input_processor::bit_lane::{pack, pack_bool, write_lane};
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};

/// First byte of the calldata header.
///
/// The whole struct is packed into a single byte, least-significant bit first:
/// each `bool` takes one bit and the trailing count takes whatever is left
/// (3 bits here). deku's `bits` feature is off, so the byte is decoded as a lane
/// in the [`bit_lane`](crate::input_processor::bit_lane) helpers.
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

impl<'a> DekuReader<'a, ()> for HeaderFlags {
    #[inline]
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        _ctx: (),
    ) -> Result<Self, DekuError> {
        let lane = read_lane::<R, 1>(reader)?;

        Ok(Self {
            read_custom_recipient: unpack_bool(lane, 0),
            read_msg_value: unpack_bool(lane, 1),
            process_dynamic_markets: unpack_bool(lane, 2),
            withdraw_eth: unpack_bool(lane, 3),
            withdraw_internally: unpack_bool(lane, 4),
            custom_erc20_count: unpack(lane, 5, 3) as usize,
        })
    }
}

#[cfg(feature = "encode")]
impl DekuWriter<()> for HeaderFlags {
    #[inline]
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        _ctx: (),
    ) -> Result<(), DekuError> {
        let mut lane = 0u64;
        lane = pack_bool(lane, self.read_custom_recipient, 0);
        lane = pack_bool(lane, self.read_msg_value, 1);
        lane = pack_bool(lane, self.process_dynamic_markets, 2);
        lane = pack_bool(lane, self.withdraw_eth, 3);
        lane = pack_bool(lane, self.withdraw_internally, 4);
        lane = pack(lane, self.custom_erc20_count as u64, 5, 3);

        write_lane::<W, 1>(writer, lane)
    }
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
