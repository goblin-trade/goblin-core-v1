use crate::{goblin_error::GoblinError, types::Address};

use super::CallPayload;

pub fn read_recipient(
    recipient_provided: bool,
    payload: &mut CallPayload,
    msg_sender: &Address,
) -> Result<Address, GoblinError> {
    if recipient_provided {
        payload.decode::<Address>()
    } else {
        Ok(*msg_sender)
    }
}
