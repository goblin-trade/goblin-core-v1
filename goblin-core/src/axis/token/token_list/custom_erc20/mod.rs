pub mod custom_erc20_deltas;
pub mod custom_erc20_list;

pub use custom_erc20_deltas::*;
pub use custom_erc20_list::*;

use crate::axis::token::{CustomERC20, CustomERC20Stub, token_list::TokenList};

impl TokenList for CustomERC20 {
    type SenderDeltaList = CustomERC20Deltas;
    type DataList<'a> = CustomERC20List<'a>;
    type HardcodedStoreList = CustomERC20Stub;
}
