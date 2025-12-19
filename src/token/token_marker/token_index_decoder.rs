use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    token::{DynamicIndex, TokenMarker, ERC20, ETH},
};

impl Decodable<<ETH as TokenMarker>::TokenIndex<DynamicIndex>> for ETH {
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<ETH as TokenMarker>::TokenIndex<DynamicIndex>, GoblinError> {
        Ok(())
    }
}

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
