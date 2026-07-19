use crate::{
    axis::token::{token_list::TokenList, CustomERC20, HardcodedERC20, Token, ETH},
    types::Triple,
};

pub type TokenDataTriple<'a> = Triple<
    <ETH as TokenList>::DataList<'a>,
    <HardcodedERC20 as TokenList>::DataList<'a>,
    <CustomERC20 as TokenList>::DataList<'a>,
    Token,
>;
