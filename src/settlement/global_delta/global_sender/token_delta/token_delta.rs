use crate::{
    axis::{
        leg::{leg_reader::LegReader, SamePair},
        token::{
            token_marker::{TokenData, TokenMarker},
            token_msg_transfer::TokenMsgTransfer,
            token_quantity::TokenQuantity,
        },
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    quantities::{UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot},
    settlement::local_delta::LocalDelta,
    state::{Preimage, StorePreimage},
    types::Address,
};

#[derive(Clone, Copy, PartialEq)]
pub struct TokenDelta<T: TokenQuantity> {
    pub deposit: T::GlobalDeposit,
    pub take: UnsidedDeltaAtoms,
    pub make: UnsidedDeltaAtoms,
}

impl<T: TokenMarker> TokenDelta<T> {
    pub fn net_delta(&self) -> UnsidedDeltaAtoms {
        self.deposit.into() + self.take + self.make
    }

    pub fn from_local_delta<In: LegReader>(
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
        local_delta: &LocalDelta,
    ) -> Self {
        let atoms_per_lot = In::get(atoms_per_lot_pair);

        let local_deposit = T::get(In::get_leg(&local_delta.deposits));
        let deposit = T::get_global_deposit(local_deposit, atoms_per_lot);

        let local_take = In::get(&local_delta.take.sender);
        let take = local_take * atoms_per_lot;

        let local_make = In::get(&local_delta.make.inner);
        let make = local_make * atoms_per_lot;

        Self {
            deposit,
            take,
            make,
        }
    }

    pub fn settle(
        &self,
        trader: &Address,
        token_data: &TokenData<T>,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError> {
        let msg_transfer = T::get_leg(msg_transfers);

        // 1. Update store
        let store_hash = StorePreimage::<T> {
            trader: *trader,
            token_address: token_data.address,
        }
        .hash();
        let mut store = store_hash.load();
        store.update(token_data, self, msg_transfer)?;
        store_hash.store(&store);

        // 2. Transfer tokens
        let net_deposit = self.deposit.into() + msg_transfer.deposit_due()?;
        UpdateEnum::transfer::<T>(net_deposit, trader, &token_data.address, store.decimals)
    }
}
