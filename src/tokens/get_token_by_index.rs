use crate::{goblin_error::GoblinError, types::Address};

use super::HARDCODED_TOKENS;

pub fn get_token_by_index(
    custom_token_list: &[Address],
    index: usize,
) -> Result<Address, GoblinError> {
    if index < custom_token_list.len() {
        Ok(custom_token_list[index])
    } else if index > 127 && index < (127 + HARDCODED_TOKENS.len()) {
        Ok(HARDCODED_TOKENS[index - 127])
    } else {
        Err(GoblinError::NoTokenAtIndex)
    }
}
