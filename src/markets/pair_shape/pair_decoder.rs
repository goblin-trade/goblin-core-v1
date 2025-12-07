use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    markets::PairShape,
    quantities::DeltaAtoms,
    require,
    token::DynamicIndex,
    token::{ERC20, ETH},
    types::Pair,
};

impl Decodable<<Pair<ETH, ERC20> as PairShape>::ResolvedPair<DynamicIndex>> for Pair<ETH, ERC20> {
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ETH, ERC20> as PairShape>::ResolvedPair<DynamicIndex>, GoblinError> {
        let byte_quote = args.decode::<u8>(offset, len)?;
        DynamicIndex::new(byte_quote)
    }
}

impl Decodable<<Pair<ERC20, ETH> as PairShape>::ResolvedPair<DynamicIndex>> for Pair<ERC20, ETH> {
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ERC20, ETH> as PairShape>::ResolvedPair<DynamicIndex>, GoblinError> {
        let byte_quote = args.decode::<u8>(offset, len)?;
        DynamicIndex::new(byte_quote)
    }
}

impl Decodable<<Pair<ERC20, ERC20> as PairShape>::ResolvedPair<DynamicIndex>>
    for Pair<ERC20, ERC20>
{
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ERC20, ERC20> as PairShape>::ResolvedPair<DynamicIndex>, GoblinError> {
        let byte_base = args.decode::<u8>(offset, len)?;
        let byte_quote = args.decode::<u8>(offset, len)?;

        require!(byte_base != byte_quote, GoblinError::InvalidTokenPair);

        let base_index = DynamicIndex::new(byte_base)?;
        let quote_index = DynamicIndex::new(byte_quote)?;

        Ok(Pair {
            base: base_index,
            quote: quote_index,
        })
    }
}

// Reuse the generic and trait to decode deposit and withdraw amounts too
impl Decodable<<Pair<ETH, ERC20> as PairShape>::ResolvedPair<DeltaAtoms>> for Pair<ETH, ERC20> {
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ETH, ERC20> as PairShape>::ResolvedPair<DeltaAtoms>, GoblinError> {
        args.decode::<i64>(offset, len).map(DeltaAtoms::new)
    }
}

impl Decodable<<Pair<ERC20, ETH> as PairShape>::ResolvedPair<DeltaAtoms>> for Pair<ERC20, ETH> {
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ERC20, ETH> as PairShape>::ResolvedPair<DeltaAtoms>, GoblinError> {
        args.decode::<i64>(offset, len).map(DeltaAtoms::new)
    }
}

impl Decodable<<Pair<ERC20, ERC20> as PairShape>::ResolvedPair<DeltaAtoms>> for Pair<ERC20, ERC20> {
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ERC20, ERC20> as PairShape>::ResolvedPair<DeltaAtoms>, GoblinError> {
        Ok(Pair {
            base: args.decode::<i64>(offset, len).map(DeltaAtoms::new)?,
            quote: args.decode::<i64>(offset, len).map(DeltaAtoms::new)?,
        })
    }
}
