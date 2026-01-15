use crate::{
    goblin_error::GoblinError,
    require,
    token::{
        CustomERC20, CustomERC20Store, DynamicIndex, ERC20Data, ERC20Marker, HardcodedERC20,
        HardcodedERC20Store, TokenIndex, HARDCODED_TOKENS,
    },
    types::Address,
};

// #[derive(Clone, Copy, PartialEq)]
// pub struct DynamicERC20;

// if we don't use enum, we need to check bits manually in places
// where we split between custom and hardcoded. Eg. in settlement delta stores.
// Not using enum is inconvenient.

// pub enum DynamicERC20Store {
//     Hardcoded(HardcodedERC20Store),
//     Custom(CustomERC20Store),
// }

// impl ERC20Data for DynamicERC20Store {
//     fn address(&self) -> &Address {
//         match self {
//             DynamicERC20Store::Hardcoded(hardcoded_erc20_store) => &hardcoded_erc20_store.address,
//             DynamicERC20Store::Custom(custom_erc20_store) => &custom_erc20_store.address,
//         }
//     }

//     fn decimals(&self) -> Result<u8, GoblinError> {
//         match self {
//             DynamicERC20Store::Hardcoded(hardcoded_erc20_store) => hardcoded_erc20_store.decimals(),
//             DynamicERC20Store::Custom(custom_erc20_store) => custom_erc20_store.decimals(),
//         }
//     }
// }

// // problem- DynamicIndex is an 'index' not a marker type
// impl ERC20Marker for DynamicIndex {
//     type Store = Address;

//     // wrong- we can't use TokenIndex<DynamicIndex>
//     // workaround- use normal marker type
//     // TokenIndex<DynamicERC20> to have a function that returns the underlying enum
//     //
//     // Unsafe hack- use union type as Store- union of pointers to hardcoded store
//     // or custom store
//     //
//     // Or revert to non-marker approach? Unsafe and union should ideally be avoided.
//     fn get_token(
//         token_index: TokenIndex<Self>,
//         custom_erc20_list: &[CustomERC20Store],
//     ) -> Result<&Self::Store, GoblinError> {
//         let byte = token_index.inner;
//         const CUSTOM_FLAG: u8 = 0b1000_0000;
//         if (byte & CUSTOM_FLAG) == 0 {
//             // Hardcoded token
//             let index_raw = byte;
//             require!(
//                 (index_raw as usize) < HARDCODED_TOKENS.len(),
//                 GoblinError::InvalidHardcodedTokenIndex
//             );

//             let hardcoded_index = TokenIndex::<HardcodedERC20>::new(index_raw);

//             let hardcoded_store =
//                 <HardcodedERC20 as ERC20Marker>::get_token(hardcoded_index, custom_erc20_list)?;

//             Ok(&hardcoded_store.address)
//         } else {
//             // Custom token
//             let index_raw = byte & !CUSTOM_FLAG; // remove the flag
//             let custom_index = TokenIndex::<CustomERC20>::new(index_raw);
//             let custom_store =
//                 <CustomERC20 as ERC20Marker>::get_token(custom_index, custom_erc20_list)?;
//             Ok(&custom_store.address)
//         }
//     }
// }
