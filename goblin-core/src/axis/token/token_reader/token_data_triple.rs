use crate::{
    axis::token::{
        token_list::{
            custom_erc20::CustomERC20List, hardcoded_erc20::HARDCODED_ERC20_LIST, TokenList,
        },
        token_marker::TokenData,
        CustomERC20, HardcodedERC20, Token, ETH,
    },
    axis_helpers::PairLeg,
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
    pub fn get_data<PL: PairLeg>(
        &self,
        token_index_pair: &TokenIndexPair<PL::Pair>,
    ) -> TokenData<PL::Selected> {
        let index = PL::Leg::get(token_index_pair);
        let data_list = PL::Selected::get_lifetimed(self);

        data_list[index]
    }
}
