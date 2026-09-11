use crate::{
    axis::token::{
        CustomERC20, ETH, HardcodedERC20, Token,
        token_list::{
            TokenList, custom_erc20::CustomERC20List, hardcoded_erc20::HARDCODED_ERC20_LIST,
        },
        token_marker::TokenData,
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

impl<'a> TokenDataTriple<'a> {
    pub const fn const_from(value: CustomERC20List<'a>) -> Self {
        Triple::new(&TokenData::<ETH>::DEFAULT, &HARDCODED_ERC20_LIST, value)
    }
}

impl<'a> TokenDataTriple<'a> {
    pub const fn get_data<PL: PairLeg>(
        &self,
        token_index_pair: &TokenIndexPair<PL::Pair>,
    ) -> TokenData<PL::Selected> {
        let index = PL::Leg::get(token_index_pair);
        let data_list = PL::Selected::get_lifetimed(self);

        data_list[index]
    }
}
