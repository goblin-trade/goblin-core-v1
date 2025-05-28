use crate::{goblin_error::GoblinError, require, types::Address};

use super::CallPayload;

pub fn read_custom_tokens<'a>(
    custom_token_count: usize,
    payload: &'a mut CallPayload,
) -> Result<&'a [Address], GoblinError> {
    let start_index = payload.offset;

    let list_len = custom_token_count * 20;
    payload.offset += list_len;

    require!(payload.len >= payload.offset, GoblinError::InvalidPayload);

    let custom_tokens = unsafe {
        core::slice::from_raw_parts(
            payload.input[start_index..payload.offset].as_ptr() as *const [u8; 20],
            custom_token_count,
        )
    };

    Ok(custom_tokens)
}
