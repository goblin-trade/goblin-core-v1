use crate::{
    axis::token::{
        token_list::{
            custom_erc20::CustomERC20List, hardcoded_erc20::HARDCODED_ERC20_LIST, TokenList,
        },
        token_marker::TokenData,
        CustomERC20, HardcodedERC20, Token, ETH,
    },
    settlement::ConstZero,
    types::Triple,
};

pub type TokenDataTriple<'a> = Triple<
    <ETH as TokenList>::DataList<'a>,
    <HardcodedERC20 as TokenList>::DataList<'a>,
    <CustomERC20 as TokenList>::DataList<'a>,
    Token,
>;

impl<'a> From<CustomERC20List<'a>> for TokenDataTriple<'a> {
    fn from(value: CustomERC20List<'a>) -> Self {
        Triple::new(&TokenData::<ETH>::ZEROED, &HARDCODED_ERC20_LIST, value)
    }
}
