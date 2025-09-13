///! Arguments read from calldata.
///!
///! * We use a custom deserialization format that begins with a header. The header tells how bytes should be interpreted.
///! For example args can contain an optional fields like `recipient` address and `eth_withdrawal_due`,
///! along with slices of variable length.
///!
///! *
use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Header},
    markets::{IndexedMarketV2, MarketInstructions},
    quantities::Atoms,
    settlement::ERC20DepositInput,
    types::Address,
};

pub struct Args<'a> {
    /// Flags and counts
    pub header: Header,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Amount of ETH to withdraw
    pub eth_withdrawal_due: Option<&'a Atoms>,

    /// Addresses of custom erc20 tokens to use
    pub custom_erc20_list: &'a [Address],

    /// Pending ERC20 deposits or withdrawals
    pub erc20_delta_list: &'a [ERC20DepositInput],

    /// Custom markets to use
    pub custom_market_list: &'a [IndexedMarketV2],

    /// List of market namespaced instructions
    pub market_instructions_list: &'a [MarketInstructions],

    pub offset: usize,
}

impl<'a> Args<'a> {
    pub fn new(payload: &'a ArgsBuffer, len: usize) -> Result<Self, GoblinError> {
        let header = Header::init(payload, len)?;
        let mut offset = Header::HEADER_BYTE_SIZE;

        let provided_recipient = header
            .recipient_provided
            .then(|| payload.decode_ref_unchecked::<Address>(&mut offset));

        let eth_withdrawal_due = header
            .track_eth_withdrawal_due
            .then(|| payload.decode_ref_unchecked::<Atoms>(&mut offset));

        let custom_erc20_list =
            payload.decode_slice_unchecked::<Address>(&mut offset, header.custom_erc20_count);
        let erc20_delta_list = payload
            .decode_slice_unchecked::<ERC20DepositInput>(&mut offset, header.erc20_deposit_count);
        let custom_market_list = payload
            .decode_slice_unchecked::<IndexedMarketV2>(&mut offset, header.custom_market_count);
        let market_instructions_list = payload.decode_slice_unchecked::<MarketInstructions>(
            &mut offset,
            header.market_instructions_count,
        );

        Ok(Args {
            header,
            recipient: provided_recipient,
            eth_withdrawal_due,
            custom_erc20_list,
            erc20_delta_list,
            custom_market_list,
            market_instructions_list,
            offset,
        })
    }
}
