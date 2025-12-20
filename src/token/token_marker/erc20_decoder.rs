use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    quantities::DeltaAtoms,
    token::{DynamicIndex, TokenMarker, ERC20},
};

impl Decodable<<ERC20 as TokenMarker>::TokenIndex<DynamicIndex>> for ERC20 {
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<ERC20 as TokenMarker>::TokenIndex<DynamicIndex>, GoblinError> {
        let index_raw = args.decode::<u8>(offset, len)?;
        Ok(DynamicIndex::new(index_raw))
    }
}

impl Decodable<<ERC20 as TokenMarker>::Deposit> for ERC20 {
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<ERC20 as TokenMarker>::Deposit, GoblinError> {
        args.decode::<i64>(offset, len).map(DeltaAtoms::new)
    }
}
