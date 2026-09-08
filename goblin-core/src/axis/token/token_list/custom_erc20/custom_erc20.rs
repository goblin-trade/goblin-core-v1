use crate::axis::token::{
    CustomERC20, CustomERC20Stub,
    token_list::{
        TokenList,
        custom_erc20::{CustomERC20Deltas, CustomERC20List},
    },
};

impl TokenList for CustomERC20 {
    type SenderDeltaList = CustomERC20Deltas;
    type DataList<'a> = CustomERC20List<'a>;
    type HardcodedStoreList = CustomERC20Stub;
}
