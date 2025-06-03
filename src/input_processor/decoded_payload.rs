use crate::{
    goblin_error::GoblinError,
    input_processor::{CallHeader, CallPayload},
    quantities::Atoms,
    require,
    settlement::TokenWithdrawalDue,
    types::Address,
};

pub struct DecodedPayload<'a> {
    pub header: CallHeader,
    pub provided_recipient: Option<&'a Address>,
    pub eth_withdrawal_due: Option<&'a Atoms>,
    pub custom_token_list: &'a [Address],
    pub token_delta_list: &'a [TokenWithdrawalDue],
}

impl<'a> DecodedPayload<'a> {
    pub fn new(payload: &'a CallPayload) -> Result<Self, GoblinError> {
        let mut offset = CallHeader::HEADER_BYTE_SIZE;

        require!(payload.len >= offset, GoblinError::InvalidPayload);
        let input = unsafe { payload.input.as_ref() };
        let header = CallHeader::init(input);

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

        let token_delta_list = {
            let count = header.token_delta_count;
            let value = payload.decode_slice::<TokenWithdrawalDue>(offset, count);
            offset += count * core::mem::size_of::<TokenWithdrawalDue>();
            value
        };

        Ok(DecodedPayload {
            header,
            provided_recipient,
            eth_withdrawal_due,
            custom_token_list,
            token_delta_list,
        })
    }
}
