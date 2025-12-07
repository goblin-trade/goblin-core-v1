use crate::token::{HardcodedToken, TokenIndex, HARDCODED_TOKENS};

pub type HardcodedIndex = TokenIndex<HardcodedToken>;

impl HardcodedIndex {
    // Return the hardcoded token address corresponding to this index
    //
    // Externally ensure that the token index is valid. This function does
    // not check for bounds.
    pub fn get_token(&self) -> &HardcodedToken {
        unsafe { HARDCODED_TOKENS.get_unchecked(self.inner as usize) }
    }
}
