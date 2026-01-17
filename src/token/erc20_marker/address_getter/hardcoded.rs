// use crate::{
//     goblin_error::GoblinError,
//     token::{AddressMapper, CustomERC20Data, ERC20Marker, HardcodedERC20, ERC20Index},
//     types::Address,
// };

// impl AddressMapper for ERC20Index<HardcodedERC20> {
//     fn address(self, custom_erc20_list: &[CustomERC20Data]) -> Result<Address, GoblinError> {
//         let store = HardcodedERC20::get_data(self, custom_erc20_list)?;
//         Ok(store.address)
//     }
// }
