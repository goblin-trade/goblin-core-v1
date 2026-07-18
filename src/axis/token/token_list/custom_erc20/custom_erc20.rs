use crate::axis::token::{
    token_list::{
        custom_erc20::{CustomERC20Deltas, CustomERC20List},
        TokenList,
    },
    CustomERC20,
};

impl TokenList for CustomERC20 {
    type SenderDeltaList = CustomERC20Deltas;
    type DataList<'a> = CustomERC20List<'a>;
}
