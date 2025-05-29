use crate::{
    goblin_error::GoblinError,
    require,
    settlement::{IndexedTokenDelta, TokenDeltaList, MAX_DELTAS},
};

use super::CallPayload;

pub fn read_token_deltas<'a>(
    payload: &'a mut CallPayload,
    token_delta_count: usize,
) -> Result<&'a [IndexedTokenDelta], GoblinError> {
    payload.decode_slice::<IndexedTokenDelta>(token_delta_count)
}
