pub mod custom_erc20;
pub mod eth;
pub mod hardcoded_erc20;

pub use custom_erc20::*;
pub use eth::*;
pub use hardcoded_erc20::*;

use core::ops::{Index, IndexMut};

use crate::{
    axis::token::{token_marker::TokenData, token_quantity::TokenQuantity},
    settlement::global_delta::TokenDelta,
};

pub trait TokenList: TokenQuantity {
    type SenderDeltaList: Index<Self::TokenIndex, Output = TokenDelta<Self>>
        + IndexMut<Self::TokenIndex>;

    type DataList<'a>: const Index<Self::TokenIndex, Output = TokenData<Self>>
        + IntoIterator<Item = &'a TokenData<Self>>
        + Copy;

    type HardcodedStoreList;
}
