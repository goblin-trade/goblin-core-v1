use crate::{
    axis::token::{
        token_list::TokenList, token_quantity::TokenQuantity, token_reader::TokenDataTriple,
        CustomERC20, HardcodedERC20, ETH,
    },
    input_processor::MsgTransfers,
    settlement::{
        global_delta::{CounterpartyMap, CounterpartyTriple, GlobalSender},
        local_delta::DepositTriple,
    },
    types::StoreReader,
};

pub trait TokenReader:
    TokenQuantity
    + TokenList
    + StoreReader<DepositTriple, Result = Self::LocalDeposit>
    + StoreReader<CounterpartyTriple, Result = CounterpartyMap<Self>>
    + StoreReader<MsgTransfers, Result = Self::TokenMsgTransfer>
    + StoreReader<GlobalSender, Result = Self::SenderDeltaList>
{
    fn get_token_data_list<'a>(store: TokenDataTriple<'a>) -> Self::DataList<'a>
    where
        Self: StoreReader<TokenDataTriple<'a>, Result = Self::DataList<'a>>,
    {
        Self::get(&store)
    }
}

impl TokenReader for ETH {}
impl TokenReader for HardcodedERC20 {}
impl TokenReader for CustomERC20 {}
