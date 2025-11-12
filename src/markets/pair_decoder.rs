use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    markets::{PairShape, ERC20, ETH},
    require,
    tokens::DynamicIndex,
    types::Pair,
};

impl Decodable<<Pair<ETH, ERC20> as PairShape>::ResolvedPair<DynamicIndex>> for Pair<ETH, ERC20> {
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ETH, ERC20> as PairShape>::ResolvedPair<DynamicIndex>, GoblinError> {
        let byte_quote = payload.decode::<u8>(offset, len)?;
        DynamicIndex::new(byte_quote)
    }
}

impl Decodable<<Pair<ERC20, ETH> as PairShape>::ResolvedPair<DynamicIndex>> for Pair<ERC20, ETH> {
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ERC20, ETH> as PairShape>::ResolvedPair<DynamicIndex>, GoblinError> {
        let byte_quote = payload.decode::<u8>(offset, len)?;
        DynamicIndex::new(byte_quote)
    }
}

impl Decodable<<Pair<ERC20, ERC20> as PairShape>::ResolvedPair<DynamicIndex>>
    for Pair<ERC20, ERC20>
{
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ERC20, ERC20> as PairShape>::ResolvedPair<DynamicIndex>, GoblinError> {
        let byte_base = payload.decode::<u8>(offset, len)?;
        let byte_quote = payload.decode::<u8>(offset, len)?;

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
impl Decodable<<Pair<ETH, ERC20> as PairShape>::ResolvedPair<i64>> for Pair<ETH, ERC20> {
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ETH, ERC20> as PairShape>::ResolvedPair<i64>, GoblinError> {
        payload.decode::<i64>(offset, len)
    }
}

impl Decodable<<Pair<ERC20, ETH> as PairShape>::ResolvedPair<i64>> for Pair<ERC20, ETH> {
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ERC20, ETH> as PairShape>::ResolvedPair<i64>, GoblinError> {
        payload.decode::<i64>(offset, len)
    }
}

impl Decodable<<Pair<ERC20, ERC20> as PairShape>::ResolvedPair<i64>> for Pair<ERC20, ERC20> {
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Pair<ERC20, ERC20> as PairShape>::ResolvedPair<i64>, GoblinError> {
        Ok(Pair {
            base: payload.decode::<i64>(offset, len)?,
            quote: payload.decode::<i64>(offset, len)?,
        })
    }
}
