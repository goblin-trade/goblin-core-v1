use core::ops::{Index, IndexMut};

use crate::{
    axis::token::{token_marker::TokenData, token_quantity::TokenQuantity},
    settlement::global_delta::TokenDelta,
};

pub trait TokenList: TokenQuantity {
    type SenderDeltaList: Index<Self::TokenIndex, Output = TokenDelta<Self>>
        + IndexMut<Self::TokenIndex>;

    type DataList<'a>: Index<Self::TokenIndex, Output = TokenData<Self>>;
}
