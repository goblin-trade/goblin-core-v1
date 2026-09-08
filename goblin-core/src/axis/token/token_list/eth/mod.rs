pub mod eth_data;
pub mod eth_delta;
pub mod eth_store_list;

pub use eth_data::*;
pub use eth_delta::*;
pub use eth_store_list::*;

use crate::axis::token::{ETH, token_marker::TokenData};

use super::TokenList;

impl TokenList for ETH {
    type SenderDeltaList = ETHDelta;
    type DataList<'a> = &'a TokenData<ETH>;
    type HardcodedStoreList = ETHStoreList;
}
