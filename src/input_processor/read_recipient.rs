use crate::{goblin_error::GoblinError, hostio_buffer::HostioBuffer, types::Address};

use super::CallPayload;

pub fn read_recipient(
    payload: &mut CallPayload,
    recipient_provided: bool,
    msg_sender: &Address,
) -> Result<Address, GoblinError> {
    if recipient_provided {
        payload.decode::<Address>()
    } else {
        Ok(*msg_sender)
    }
}
