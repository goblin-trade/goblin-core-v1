// use crate::{
//     goblin_error::GoblinError,
//     token::{
//         AddressMapper, CustomERC20, CustomERC20Data, DynamicIndex, ERC20Index, HardcodedERC20,
//     },
//     types::Address,
// };

// impl AddressMapper for DynamicIndex {
//     fn address(self, custom_erc20_list: &[CustomERC20Data]) -> Result<Address, GoblinError> {
//         match self {
//             DynamicIndex::Hardcoded(erc20_index) => {
//                 ERC20Index::<HardcodedERC20>::address(erc20_index, custom_erc20_list)
//             }
//             DynamicIndex::Custom(erc20_index) => {
//                 ERC20Index::<CustomERC20>::address(erc20_index, custom_erc20_list)
//             }
//         }
//     }
// }
