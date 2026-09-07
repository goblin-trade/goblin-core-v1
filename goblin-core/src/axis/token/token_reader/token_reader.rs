use crate::{
    axis::token::{
        token_list::TokenList, token_quantity::TokenQuantity, token_reader::TokenDataTriple,
        CustomERC20, HardcodedERC20, ETH,
    },
    input_processor::MsgTransfers,
    settlement::global_delta::{CounterpartyMap, CounterpartyTriple, GlobalSender},
    types::{LifetimedStoreReader, StoreReader},
};

pub trait TokenReader:
    TokenQuantity
    + TokenList
    + for<'a> LifetimedStoreReader<'a, TokenDataTriple<'a>, Result = Self::DataList<'a>>
    + StoreReader<CounterpartyTriple, Result = CounterpartyMap<Self>>
    + StoreReader<MsgTransfers, Result = Self::TokenMsgTransfer>
    + StoreReader<GlobalSender, Result = Self::SenderDeltaList>
{
}

impl TokenReader for ETH {}
impl TokenReader for HardcodedERC20 {}
impl TokenReader for CustomERC20 {}
