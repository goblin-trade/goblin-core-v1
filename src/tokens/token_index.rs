use crate::{
    tokens::{ERC20Token, HardcodedToken, HARDCODED_TOKENS},
    types::Address,
};

/// Unifying type to lookup ERC20 token address by index
/// Can we avoid a common enum type completely?
/// * Currently we use TokenIndex in the delta map. Perhaps we can use separate maps
/// for custom and hardcoded.
/// * But this means extra list for maker deltas too
#[derive(Clone, Copy)]
pub enum DynamicTokenIndex {
    Hardcoded(HardcodedIndex),
    Custom(CustomIndex),
}

impl DynamicTokenIndex {
    pub fn get_token(&self, custom_erc20_list: &[Address]) -> Option<ERC20Token> {
        match self {
            DynamicTokenIndex::Hardcoded(hardcoded_index) => hardcoded_index
                .get_token()
                .map(|token| ERC20Token::Hardcoded(*token)),
            DynamicTokenIndex::Custom(custom_index) => custom_index
                .get_token(custom_erc20_list)
                .map(|token| ERC20Token::Custom(*token)),
        }
    }
}

/// Index to lookup ERC20 token from the hardcoded list
/// These tokens have their address and decimal count hardcoded
#[derive(Clone, Copy)]
pub struct HardcodedIndex(pub u8);

impl HardcodedIndex {
    pub fn get_token(&self) -> Option<&HardcodedToken> {
        HARDCODED_TOKENS.get(self.0 as usize)
    }
}

/// Index to lookup ERC20 token address from custom erc20 list
#[derive(Clone, Copy)]
pub struct CustomIndex(pub u8);

impl CustomIndex {
    pub fn get_token<'a>(&self, custom_erc20_list: &'a [Address]) -> Option<&'a Address> {
        custom_erc20_list.get(self.0 as usize)
    }
}

// #[repr(C, packed)]
// #[derive(Clone, Copy, PartialEq)]
// pub struct TokenIndex(pub u8);

// impl TokenIndex {
//     pub const ETH: Self = TokenIndex(255);

//     /// Obtain the token corresponding to the token index.
//     /// Token can be ETH or an ERC20
//     ///
//     /// Token index to token lookup happens in two places
//     /// * To obtain slot keys for a market
//     /// * In the settlement phase for token transfers and slot updates.
//     ///
//     /// This is still cheap because we're looking up values for an index within a slice.
//     /// There is no need to store token address in ERC20Delta
//     ///
//     /// In the intermediary stages, it is possible that an invalid token index is passed to ERC20Delta.
//     /// However this is caught during settlement.
//     pub fn to_token(&self, custom_erc20_list: &[Address]) -> Result<Token, GoblinError> {
//         if *self == Self::ETH {
//             return Ok(Token::Eth);
//         }

//         let index = self.0 as usize;

//         if index < custom_erc20_list.len() {
//             let token = custom_erc20_list[index];
//             Ok(Token::ERC20(ERC20Token::Custom(token)))
//         } else if index > 127 && index < (127 + HARDCODED_TOKENS.len()) {
//             let token = HARDCODED_TOKENS[index - 127];
//             Ok(Token::ERC20(ERC20Token::Hardcoded(token)))
//         } else {
//             Err(GoblinError::NoTokenAtIndex)
//         }
//     }
// }
