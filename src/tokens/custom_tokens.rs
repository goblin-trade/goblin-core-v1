use crate::{goblin_error::GoblinError, require, types::Address};

const START_INDEX: usize = 4;

pub fn read_custom_tokens<'a>(
    custom_token_count: u8,
    input: &'a [u8; 512],
    len: usize,
) -> Result<&'a [[u8; 20]], GoblinError> {
    let list_len = custom_token_count as usize * core::mem::size_of::<Address>();
    let total_len = START_INDEX + list_len; // 24
    require!(len >= total_len, GoblinError::InvalidPayload);

    // end index exclusive = len
    let custom_tokens = unsafe {
        core::slice::from_raw_parts(
            input[START_INDEX..total_len].as_ptr() as *const [u8; 20],
            custom_token_count as usize,
        )
    };

    Ok(custom_tokens)
}
