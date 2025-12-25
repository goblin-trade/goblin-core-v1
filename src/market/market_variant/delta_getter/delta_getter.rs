use crate::{
    market::MarketVariant,
    settlement::global_delta::{
        ERC20Delta, ERC20MakerDeltas, ERC20SenderDeltas, UnsidedMakerDelta,
    },
    types::Address,
};

/// Get deltas for a market variant
pub trait DeltaGetter: MarketVariant {
    fn token_sender_delta_mut(
        token_index: Self::TokenIndex,
        token_sender_deltas: &mut ERC20SenderDeltas,
    ) -> &mut ERC20Delta;

    fn token_maker_delta_mut(
        token_index: Self::TokenIndex,
        maker: Address,
        token_maker_deltas: &mut ERC20MakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta>;
}
