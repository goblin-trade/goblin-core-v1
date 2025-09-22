use crate::{
    markets::{IndexedMarket, MarketLeg},
    matching::SenderSideDelta,
    quantities::{Atoms, BaseLotsPerBaseUnit},
    types::{Base, LegMarker, Quote},
};

#[derive(Default)]
pub struct SenderDelta {
    pub take_base_in: SenderSideDelta<Base>,
    pub take_quote_in: SenderSideDelta<Quote>,
    // Add limit order and cancel fields later
}

// Pending updates for the taker per token after performing
// taker ask and quote trades on a market
pub struct TakerTokenUpdate {
    pub free_atoms_in: Atoms,
    pub locked_atoms_out: Atoms,
    pub atoms_released_by_self_trade: Atoms,
}

impl SenderDelta {
    pub fn to_taker_token_update<In: LegMarker>(
        &self,
        market_leg: &MarketLeg<In>,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> TakerTokenUpdate {
        let atoms_per_lot = In::atoms_per_lot(market_leg.lot_size);

        let delta = In::sender_side_delta(self);
        let delta_opposite = <In::Opposite as LegMarker>::sender_side_delta(self);

        let free_atoms_in = In::matching_lots_to_atoms(
            delta.maker_side_delta.free_matching_lots_in,
            base_lot_size,
            atoms_per_lot,
        );

        let locked_atoms_out = In::matching_lots_to_atoms(
            delta_opposite.maker_side_delta.locked_matching_lots_out,
            base_lot_size,
            atoms_per_lot,
        );

        let atoms_released_by_self_trade = In::matching_lots_to_atoms(
            delta_opposite.released_by_self_trade,
            base_lot_size,
            atoms_per_lot,
        );

        TakerTokenUpdate {
            free_atoms_in,
            locked_atoms_out,
            atoms_released_by_self_trade,
        }
    }
}
