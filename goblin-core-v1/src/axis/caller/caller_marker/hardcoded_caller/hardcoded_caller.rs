use crate::{
    axis::{
        token_marker::ETH_STORE_HASH_LIST, CallerMarker, HardcodedCaller, HardcodedCallerIndex,
        TokenEnum, TokenMarker,
    },
    state::{Preimage, SlotKey, StorePreimage},
    types::Address,
};

impl CallerMarker for HardcodedCaller {
    type CallerIndex = HardcodedCallerIndex;

    // fn get_store_hash<TM: TokenMarker>(
    //     preimage: &StorePreimage<TM>,
    //     token_index: TM::TokenIndex,
    //     caller_index: Self::CallerIndex,
    // ) -> SlotKey<StorePreimage<TM>> {
    //     match TM::VARIANT {
    //         TokenEnum::ETH => ETH_STORE_HASH_LIST[caller_index.inner],
    //         TokenEnum::HardcodedERC20 => todo!(),
    //         TokenEnum::CustomERC20 => preimage.hash(),
    //     }
    // }
}
