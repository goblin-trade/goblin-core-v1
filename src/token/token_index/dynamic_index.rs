use crate::{
    goblin_error::GoblinError,
    require,
    token::{
        CustomERC20, CustomERC20Store, ERC20Marker, HardcodedERC20, TokenIndex, HARDCODED_TOKENS,
    },
    types::Address,
};

/// Enum type of hardcoded and custom token indices.
///
/// Allows custom markets to use both hardcoded and custom tokens
#[derive(Clone, Copy, PartialEq)]
pub enum DynamicIndex {
    Hardcoded(TokenIndex<HardcodedERC20>),
    Custom(TokenIndex<CustomERC20>),
}

impl DynamicIndex {
    /// Decode a token index byte into either a hardcoded or custom token index.
    /// Token indices are lazily validated when mapping to address.
    ///
    /// Convention:
    /// - If the MSB (bit 7) is 0 → Hardcoded token index (0–127)
    /// - If the MSB (bit 7) is 1 → Custom token index (0–127, but stored as 128–255)
    pub fn new(byte: u8) -> Self {
        const CUSTOM_FLAG: u8 = 0b1000_0000;
        if (byte & CUSTOM_FLAG) == 0 {
            // Hardcoded token
            // let index = byte;
            // require!(
            //     (index as usize) < HARDCODED_TOKENS.len(),
            //     GoblinError::InvalidHardcodedTokenIndex
            // );

            Self::Hardcoded(TokenIndex::new(byte))
            // Ok(Self::Hardcoded(TokenIndex::new(index)))
        } else {
            // Custom token
            // inconsistency- we check bounds of hardcoded but not custom
            let index = byte & !CUSTOM_FLAG; // remove the flag
            Self::Custom(TokenIndex::new(index))
            // Ok(Self::Custom(TokenIndex::new(index)))
        }
    }

    // /// Get the token address corresponding to the index. If it is a custom token, this
    // /// address is read from the custom token list
    // ///
    // /// Problem- different APIs
    // ///
    // /// 1. <HardcodedERC20 as ERC20Marker>::get_token(hardcoded_token_index, custom_erc20_list) -> ERC20Store
    // ///
    // /// 2. dynamic_token_index.address(custom_erc20_list)
    // pub fn address(self, custom_erc20_list: &[CustomERC20Store]) -> Result<&Address, GoblinError> {
    //     let address = match self {
    //         DynamicIndex::Hardcoded(hardcoded_token_index) => {
    //             let token = <HardcodedERC20 as ERC20Marker>::get_token(
    //                 hardcoded_token_index,
    //                 custom_erc20_list,
    //             )?;
    //             <HardcodedERC20 as ERC20Marker>::address(token)
    //         }
    //         DynamicIndex::Custom(custom_token_index) => {
    //             let token =
    //                 <CustomERC20 as ERC20Marker>::get_token(custom_token_index, custom_erc20_list)?;
    //             <CustomERC20 as ERC20Marker>::address(token)
    //         }
    //     };

    //     Ok(address)
    // }
}
