use crate::{goblin_error::GoblinError, types::Address};

use super::CallPayload;

// pub fn read_custom_tokens<'a>(
//     payload: &'a mut CallPayload,
//     custom_token_count: usize,
// ) -> Result<&'a [Address], GoblinError> {
//     payload.decode_slice::<Address>(custom_token_count)
// }
