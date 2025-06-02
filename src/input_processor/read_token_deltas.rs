use crate::{
    goblin_error::GoblinError,
    require,
    settlement::{TokenWithdrawalDue, TokensConsumedList, MAX_DELTAS},
};

use super::CallPayload;

// pub fn read_token_deltas<'a>(
//     payload: &'a mut CallPayload,
//     token_delta_count: usize,
// ) -> Result<&'a [TokenWithdrawalDue], GoblinError> {
//     payload.decode_slice::<TokenWithdrawalDue>(token_delta_count)
// }
