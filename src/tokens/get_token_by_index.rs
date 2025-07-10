use crate::{goblin_error::GoblinError, tokens::HARDCODED_TOKENS, types::Address};

/// Get token address for the given index.
///
/// First look up in custom_token_list, then in hardcoded_tokens. Hardcoded tokens are indexed from 128 to 128 + HARDCODED_TOKENS.len().
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
