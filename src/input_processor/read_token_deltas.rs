use crate::{
    goblin_error::GoblinError,
    require,
    settlement::{IndexedTokenDelta, TokenDeltaList, MAX_DELTAS},
};

pub fn read_token_deltas<'a>(
    token_delta_count: usize,
    input: &'a [u8; 512],
    len: usize,
    offset: &mut usize,
) -> Result<TokenDeltaList<'a>, GoblinError> {
    let start_index = *offset;

    let byte_count = token_delta_count * 9;
    *offset += byte_count;
    require!(len >= *offset, GoblinError::InvalidPayload);

    let needed_bytes = &input[start_index..*offset];

    let mut delta_bytes = [0u8; MAX_DELTAS * core::mem::size_of::<IndexedTokenDelta>()];
    delta_bytes[..byte_count].copy_from_slice(needed_bytes);

    let deltas: &mut [IndexedTokenDelta; MAX_DELTAS] =
        unsafe { &mut *(delta_bytes.as_mut_ptr() as *mut [IndexedTokenDelta; MAX_DELTAS]) };

    let list = TokenDeltaList {
        deltas,
        len: token_delta_count,
    };

    Ok(list)
}
