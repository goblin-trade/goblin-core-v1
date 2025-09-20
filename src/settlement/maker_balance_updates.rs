use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    quantities::Atoms,
    settlement::{MakerDelta, MarketMakerDeltas},
    tokens::TokenIndex,
    types::{Address, Base, LegMarker, Quote},
    utils::FixedMap,
};

const MAX_BALANCE_UPDATES: usize = 16;
pub type MakerBalanceUpdates = FixedMap<UpdateKey, Update, MAX_BALANCE_UPDATES>;

#[derive(PartialEq, Clone, Copy)]
struct UpdateKey {
    pub maker: Address,
    pub token_index: TokenIndex,
}

#[derive(Default)]
struct Update {
    pub locked_atoms_out: Atoms,
    pub free_atoms_in: Atoms,
}

impl MakerBalanceUpdates {
    fn apply_side_update<In: LegMarker>(
        &mut self,
        indexed_market: &IndexedMarket,
        maker: &Address,
        maker_delta: &MakerDelta,
    ) -> Result<(), GoblinError> {
        let market_leg = In::market_leg(indexed_market);
        let atoms_per_lot = In::atoms_per_lot(market_leg.lot_size);

        // Free atoms in
        let maker_side_delta = In::maker_side_delta_ref(maker_delta);
        let free_lots_in = In::decode_matching_lots(
            maker_side_delta.free_matching_lots_in,
            indexed_market.base.lot_size,
        );
        let free_atoms_in: Atoms = (free_lots_in * atoms_per_lot).into();

        // Locked atoms out
        let maker_side_delta_opposite = In::Opposite::maker_side_delta_ref(maker_delta);
        let locked_lots_out = In::decode_matching_lots(
            maker_side_delta_opposite.locked_matching_lots_out,
            indexed_market.base.lot_size,
        );
        let locked_atoms_out: Atoms = (locked_lots_out * atoms_per_lot).into();

        // Write to store
        let store = self
            .get_or_insert_mut(UpdateKey {
                maker: *maker,
                token_index: market_leg.token_index,
            })
            .ok_or(GoblinError::MakerStoreListFull)?;
        store.free_atoms_in += free_atoms_in;
        store.locked_atoms_out += locked_atoms_out;

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
