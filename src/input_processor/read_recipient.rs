use crate::{goblin_error::GoblinError, require, types::Address};

use super::CallPayload;

pub fn read_recipient(
    recipient_provided: bool,
    payload: &mut CallPayload,
    msg_sender: &Address,
) -> Result<Address, GoblinError> {
    if recipient_provided {
        let start_index = payload.offset;
        payload.offset += 20;
        require!(payload.len >= payload.offset, GoblinError::InvalidPayload);

        let address =
            unsafe { *(payload.input[start_index..payload.offset].as_ptr() as *const Address) };

        Ok(address)
    } else {
        Ok(*msg_sender)
    }
}
