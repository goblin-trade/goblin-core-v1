use crate::{goblin_error::GoblinError, require, types::Address};

pub fn read_custom_tokens<'a>(
    custom_token_count: u8,
    input: &'a [u8; 512],
    len: usize,
    offset: &mut usize,
) -> Result<&'a [Address], GoblinError> {
    let start_index = *offset;

    let list_len = custom_token_count as usize * 20;
    *offset += list_len;

    require!(len >= *offset, GoblinError::InvalidPayload);

    let custom_tokens = unsafe {
        core::slice::from_raw_parts(
            input[start_index..*offset].as_ptr() as *const [u8; 20],
            custom_token_count as usize,
        )
    };

    Ok(custom_tokens)
}
