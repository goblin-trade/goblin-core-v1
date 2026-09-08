pub mod hardcoded_erc20_deltas;
pub mod hardcoded_erc20_list;
pub mod hardcoded_erc20_store_list;

pub use hardcoded_erc20_deltas::*;
pub use hardcoded_erc20_list::*;
pub use hardcoded_erc20_store_list::*;

use super::TokenList;
use crate::axis::token::HardcodedERC20;

impl TokenList for HardcodedERC20 {
    type SenderDeltaList = HardcodedERC20Deltas;
    type DataList<'a> = &'a HardcodedERC20List<HARDCODED_ERC20_COUNT>;
    type HardcodedStoreList = HardcodedERC20StoreList;
}
