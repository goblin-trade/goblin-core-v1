use crate::{goblin_error::GoblinError, require, settlement::IndexedTokenDelta};

pub fn read_token_deltas<'a>(
    token_delta_count: usize,
    input: &'a [u8; 512],
    len: usize,
    offset: &mut usize,
) -> Result<&'a [IndexedTokenDelta], GoblinError> {
    let start_index = *offset;
    *offset += token_delta_count * 9;
    require!(len >= *offset, GoblinError::InvalidPayload);

    let token_deltas = unsafe {
        core::slice::from_raw_parts(
            input[start_index..*offset].as_ptr() as *const IndexedTokenDelta,
            token_delta_count,
        )
    };

    Ok(token_deltas)
}
