use crate::axis::token::{
    token_list::{
        hardcoded_erc20::{HardcodedERC20Deltas, HardcodedERC20List, HARDCODED_ERC20_COUNT},
        TokenList,
    },
    HardcodedERC20,
};

impl TokenList for HardcodedERC20 {
    type SenderDeltaList = HardcodedERC20Deltas;
    type DataList<'a> = &'a HardcodedERC20List<HARDCODED_ERC20_COUNT>;
}
