use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    market::Dynamic,
    quantities::DeltaAtoms,
    token::{DynamicIndex, TokenMarker, ERC20},
};

impl Decodable for <ERC20 as TokenMarker>::TokenIndex<Dynamic>
where
    ERC20: TokenMarker,
{
    fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError> {
        let index_raw = args.decode::<u8>(offset, len)?;
        DynamicIndex::new(index_raw)
    }
}

impl Decodable for <ERC20 as TokenMarker>::Deposit {
    fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError> {
        args.decode::<i64>(offset, len).map(DeltaAtoms::new)
    }
}
