use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Header},
    markets::IndexedMarket,
    quantities::Atoms,
    settlement::IndexedERC20Delta,
    types::Address,
};

pub struct Args<'a> {
    /// Flags and counts
    pub header: Header,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Amount of ETH to withdraw
    pub eth_withdrawal_due: Option<&'a Atoms>,

    /// Addresses of custom tokens to use
    pub custom_token_list: &'a [Address],

    /// Pending ERC20 deposits or withdrawals
    pub erc20_delta_list: &'a [IndexedERC20Delta],

    /// Custom markets to use
    pub custom_market_list: &'a [IndexedMarket],
}

impl<'a> Args<'a> {
    pub fn new(payload: &'a ArgsBuffer, len: usize) -> Result<Self, GoblinError> {
        let header = Header::init(payload, len)?;
        let mut offset = Header::HEADER_BYTE_SIZE;

        let provided_recipient = if header.recipient_provided {
            let value = payload.decode_ref::<Address>(offset);
            offset += core::mem::size_of::<Address>();
            Some(value)
        } else {
            None
        };

        let eth_withdrawal_due = if header.track_eth_withdrawal_due {
            let value = payload.decode_ref::<Atoms>(offset);
            offset += core::mem::size_of::<Atoms>();
            Some(value)
        } else {
            None
        };

        let custom_token_list = {
            let count = header.custom_token_count;
            let value = payload.decode_slice::<Address>(offset, count);
            offset += count * core::mem::size_of::<Address>();
            value
        };

        let erc20_delta_list = {
            let count = header.erc20_delta_count;
            let value = payload.decode_slice::<IndexedERC20Delta>(offset, count);
            offset += count * core::mem::size_of::<IndexedERC20Delta>();
            value
        };

        let custom_market_list = {
            let count = header.custom_market_count;
            let value = payload.decode_slice::<IndexedMarket>(offset, count);
            offset += count * core::mem::size_of::<IndexedMarket>();
            value
        };

        Ok(Args {
            header,
            recipient: provided_recipient,
            eth_withdrawal_due,
            custom_token_list,
            erc20_delta_list,
            custom_market_list,
        })
    }
}
