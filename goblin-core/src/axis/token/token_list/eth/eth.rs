use crate::axis::{
    token::{
        token_list::{eth::ETHDelta, ETHStoreList, TokenList},
        token_marker::TokenData,
        ETH,
    },
};

impl TokenList for ETH {
    type SenderDeltaList = ETHDelta;
    type DataList<'a> = &'a TokenData<ETH>;
    type HardcodedStoreList = ETHStoreList;
}
