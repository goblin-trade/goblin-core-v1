use crate::{
    goblin_error::GoblinError, markets::IndexedMarketV2, quantities::Atoms,
    settlement::PendingMakerUpdates, tokens::TokenIndex, types::Address, utils::FixedMap,
};

pub const MAX_OPPOSITE_DELTAS: usize = 16;

#[derive(PartialEq, Clone, Copy)]
pub struct PendingStoreKey {
    pub maker: Address,
    pub token_index: TokenIndex,
}

#[derive(Default)]
pub struct PendingStoreUpdate {
    pub locked_atoms_out: Atoms,
    pub free_atoms_in: Atoms,
}

pub type PendingMakerStoreUpdates =
    FixedMap<PendingStoreKey, PendingStoreUpdate, MAX_OPPOSITE_DELTAS>;

impl PendingMakerStoreUpdates {
    pub fn apply_updates(
        &mut self,
        indexed_market: &IndexedMarketV2,
        updates: &PendingMakerUpdates,
    ) -> Result<(), GoblinError> {
        for (maker, update) in updates.iter() {
            // TODO make it compact using SideMarker
            // But each store receives contributions from both bid and ask side
            // Base / Quote namespace is separate from the side namespace.
            let base_atoms_per_base_lot = indexed_market.base.atoms_per_lot();

            let base_store = self
                .get_or_insert_mut(PendingStoreKey {
                    maker: *maker,
                    token_index: indexed_market.base.token_index,
                })
                .ok_or(GoblinError::MakerStoreListFull)?;

            // Convert BaseAtoms and QuoteAtoms to Atoms
            base_store.locked_atoms_out +=
                Atoms::from(update.bid.locked_lots_out * base_atoms_per_base_lot);
            base_store.free_atoms_in +=
                Atoms::from(update.ask.free_lots_in * base_atoms_per_base_lot);

            let quote_atoms_per_quote_lot = indexed_market.quote.atoms_per_lot();
            let quote_store = self
                .get_or_insert_mut(PendingStoreKey {
                    maker: *maker,
                    token_index: indexed_market.quote.token_index,
                })
                .ok_or(GoblinError::MakerStoreListFull)?;

            quote_store.locked_atoms_out +=
                Atoms::from(update.ask.locked_lots_out * quote_atoms_per_quote_lot);
            quote_store.free_atoms_in +=
                Atoms::from(update.bid.free_lots_in * quote_atoms_per_quote_lot);
        }

        Ok(())
    }

    pub fn settle(&self) {}
}
