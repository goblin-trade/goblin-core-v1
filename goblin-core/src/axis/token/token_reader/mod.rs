pub mod token_data_triple;

pub use token_data_triple::*;

use super::{
    CustomERC20, ETH, HardcodedERC20, token_list::TokenList, token_quantity::TokenQuantity,
};
use crate::{
    input_processor::MsgTransfers,
    settlement::global_delta::{CounterpartyMap, CounterpartyTriple, GlobalSender},
    types::{LifetimedStoreReader, StoreReader},
};

pub trait TokenReader:
    TokenQuantity
    + TokenList
    + for<'a> const LifetimedStoreReader<'a, TokenDataTriple<'a>, Result = Self::DataList<'a>>
    + StoreReader<CounterpartyTriple, Result = CounterpartyMap<Self>>
    + StoreReader<MsgTransfers, Result = Self::TokenMsgTransfer>
    + StoreReader<GlobalSender, Result = Self::SenderDeltaList>
{
}

impl TokenReader for ETH {}
impl TokenReader for HardcodedERC20 {}
impl TokenReader for CustomERC20 {}
