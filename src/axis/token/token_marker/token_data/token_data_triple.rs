use crate::{
    axis::token::{
        token_marker::{CustomERC20List, TokenMarker, HARDCODED_TOKENS},
        CustomERC20, ETHStub, HardcodedERC20, Token, ETH,
    },
    types::Triple,
};

// pub type TokenDataListTriple = Triple<
//     <ETH as TokenMarker>::DataList,
//     <HardcodedERC20 as TokenMarker>::DataList,
//     <CustomERC20 as TokenMarker>::DataList,
//     Token,
// >;

// impl TokenDataListTriple {
//     pub fn make_new(custom_erc20_list: CustomERC20List) -> Self {
//         Triple::new(ETHStub, HARDCODED_TOKENS, custom_erc20_list)
//     }
// }
