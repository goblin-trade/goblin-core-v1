use crate::axis::token::{
    HardcodedERC20,
    token_list::{
        HardcodedERC20StoreList, TokenList,
        hardcoded_erc20::{HARDCODED_ERC20_COUNT, HardcodedERC20Deltas, HardcodedERC20List},
    },
};

impl TokenList for HardcodedERC20 {
    type SenderDeltaList = HardcodedERC20Deltas;
    type DataList<'a> = &'a HardcodedERC20List<HARDCODED_ERC20_COUNT>;
    type HardcodedStoreList = HardcodedERC20StoreList;
}
