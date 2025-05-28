use crate::{
    goblin_error::GoblinError,
    require,
    settlement::{IndexedTokenDelta, TokenDeltaList, MAX_DELTAS},
};

use super::CallPayload;

pub fn read_token_deltas<'a>(
    token_delta_count: usize,
    payload: &'a mut CallPayload,
) -> Result<TokenDeltaList<'a>, GoblinError> {
    let start_index = payload.offset;

    let byte_count = token_delta_count * 9;
    payload.offset += byte_count;
    require!(payload.len >= payload.offset, GoblinError::InvalidPayload);

    let needed_bytes = &payload.input[start_index..payload.offset];

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
