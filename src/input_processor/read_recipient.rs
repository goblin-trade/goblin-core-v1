use crate::{goblin_error::GoblinError, require, types::Address};

pub fn read_recipient<'a>(
    recipient_provided: bool,
    input: &'a [u8; 512],
    len: usize,
    offset: &mut usize,
    msg_sender: &'a Address,
) -> Result<&'a Address, GoblinError> {
    let recipient = if recipient_provided {
        let start_index = *offset;
        *offset += 20;

        require!(len >= *offset, GoblinError::InvalidPayload);
        unsafe { &*(input[start_index..*offset].as_ptr() as *const Address) }
    } else {
        msg_sender
    };

    Ok(recipient)
}
