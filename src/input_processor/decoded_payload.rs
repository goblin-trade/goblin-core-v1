use crate::{
    goblin_error::GoblinError,
    input_processor::{CallHeader, CallPayload},
    quantities::Atoms,
    require,
    settlement::ERC20_WITHDRAWAL_ITEM_SIZE,
    types::Address,
};

pub struct DecodedPayload<'a> {
    /// Flags and counts
    pub header: CallHeader,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Amount of ETH to withdraw
    pub eth_withdrawal_due: Option<&'a Atoms>,

    /// Addresses of custom tokens to use, max 15
    pub custom_token_list: &'a [Address],

    /// 9 bytes for (token index, withdrawal_due_delta) for each token pending
    /// withdrawal or deposit.
    /// Max 15 withdrawals, i.e. 9 * 15 = 135 bytes
    pub erc20_withdrawals_bytes: &'a [u8],
}

impl<'a> DecodedPayload<'a> {
    pub fn new(payload: &'a CallPayload) -> Result<Self, GoblinError> {
        let mut offset = CallHeader::HEADER_BYTE_SIZE;

        require!(payload.len >= offset, GoblinError::InvalidPayload);
        let header = CallHeader::init(payload.input.as_ref());

        require!(
            payload.len >= header.payload_size(),
            GoblinError::InvalidPayload
        );

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

        let start = offset;
        offset += header.custom_token_count * ERC20_WITHDRAWAL_ITEM_SIZE;
        let erc20_withdrawals_bytes = &payload.input.as_ref()[start..offset];

        Ok(DecodedPayload {
            header,
            recipient: provided_recipient,
            eth_withdrawal_due,
            custom_token_list,
            erc20_withdrawals_bytes,
        })
    }
}
