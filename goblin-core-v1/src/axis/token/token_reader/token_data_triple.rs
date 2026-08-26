use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::{
            token_list::{
                custom_erc20::CustomERC20List, hardcoded_erc20::HARDCODED_ERC20_LIST, TokenList,
            },
            token_marker::TokenData,
            token_quantity::TokenQuantity,
            CustomERC20, HardcodedERC20, Token, ETH,
        },
    },
    axis_helpers::{LegToToken, TokenPair},
    market::TokenIndexPair,
    settlement::ConstDefault,
    types::{LifetimedStoreReader, StoreReader, Triple},
};

pub type TokenDataTriple<'a> = Triple<
    <ETH as TokenList>::DataList<'a>,
    <HardcodedERC20 as TokenList>::DataList<'a>,
    <CustomERC20 as TokenList>::DataList<'a>,
    Token,
>;

impl<'a> From<CustomERC20List<'a>> for TokenDataTriple<'a> {
    fn from(value: CustomERC20List<'a>) -> Self {
        Triple::new(&TokenData::<ETH>::DEFAULT, &HARDCODED_ERC20_LIST, value)
    }
}

impl<'a> TokenDataTriple<'a> {
    pub fn get_data<TP, In>(
        &self,
        token_index_pair: &TokenIndexPair<TP>,
    ) -> TokenData<<In as LegToToken<TP>>::Selected>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>,
    {
        let index = In::get(token_index_pair);
        let data_list = In::Selected::get_lifetimed(self);

        data_list[index]
    }
}
