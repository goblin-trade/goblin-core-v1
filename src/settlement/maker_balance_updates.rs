use crate::{
    goblin_error::GoblinError,
    markets::{IndexedMarket, MarketLeg},
    quantities::{Atoms, BaseLotsPerBaseUnit},
    settlement::{MakerDelta, MarketMakerDeltas},
    tokens::TokenIndex,
    types::{Address, Base, LegMarker, Quote},
    utils::FixedMap,
};

const MAX_BALANCE_UPDATES: usize = 16;
pub type MakerBalanceUpdates = FixedMap<UpdateKey, Update, MAX_BALANCE_UPDATES>;

#[derive(PartialEq, Clone, Copy)]
pub struct UpdateKey {
    pub maker: Address,
    pub token_index: TokenIndex,
}

#[derive(Default)]
pub struct Update {
    pub free_atoms_in: Atoms,
    pub locked_atoms_out: Atoms,
}

impl Update {
    fn new<In: LegMarker>(
        maker_delta: &MakerDelta,
        market_leg: &MarketLeg<In>,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self {
        let atoms_per_lot = In::atoms_per_lot(market_leg.lot_size);

        // Free atoms in
        let maker_side_delta = In::maker_side_delta_ref(maker_delta);
        let maker_side_delta_opposite = In::Opposite::maker_side_delta_ref(maker_delta);

        let free_atoms_in = In::matching_lots_to_atoms(
            maker_side_delta.free_matching_lots_in,
            base_lot_size,
            atoms_per_lot,
        );

        let locked_atoms_out = In::matching_lots_to_atoms(
            maker_side_delta_opposite.locked_matching_lots_out,
            base_lot_size,
            atoms_per_lot,
        );

        Self {
            locked_atoms_out,
            free_atoms_in,
        }
    }
}

impl MakerBalanceUpdates {
    fn apply_side_update<In: LegMarker>(
        &mut self,
        indexed_market: &IndexedMarket,
        maker: &Address,
        maker_delta: &MakerDelta,
    ) -> Result<(), GoblinError> {
        let base_lot_size = indexed_market.base.lot_size;
        let market_leg = In::market_leg(indexed_market);
        let update = Update::new::<In>(maker_delta, market_leg, base_lot_size);

        // Write to store
        let store = self
            .get_or_insert_mut(UpdateKey {
                maker: *maker,
                token_index: market_leg.token_index,
            })
            .ok_or(GoblinError::MakerStoreListFull)?;
        store.free_atoms_in += update.free_atoms_in;
        store.locked_atoms_out += update.locked_atoms_out;

        Ok(())
    }

    pub fn apply_updates(
        &mut self,
        indexed_market: &IndexedMarket,
        deltas: &MarketMakerDeltas,
    ) -> Result<(), GoblinError> {
        for (maker, maker_delta) in deltas.iter() {
            self.apply_side_update::<Base>(indexed_market, maker, maker_delta)?;
            self.apply_side_update::<Quote>(indexed_market, maker, maker_delta)?;
        }

        Ok(())
    }

    pub fn settle(&self) {}
}
