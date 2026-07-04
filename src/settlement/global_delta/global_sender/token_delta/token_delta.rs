use crate::{
    axis::{
        leg::{leg_reader::LegReader, SamePair},
        token::{
            token_deltas::TokenDeltas,
            token_global_transfer::{ETHTransfers, TokenGlobalTransfer},
            token_index::TokenIndex,
            token_marker::TokenMarker,
        },
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    quantities::{IntoAbs, UnsidedAtoms, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot},
    settlement::local_delta::LocalDelta,
    state::{Preimage, StorePreimage},
    types::Address,
};

#[derive(Clone, Copy)]
pub struct TokenDelta<T: TokenMarker> {
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
        token_index: T::TokenIndex,
        token_address: <T::TokenIndex as TokenIndex>::TokenAddress,
        trader: Address,
        global_transfer: T::TokenGlobalTransfer,
    ) -> Result<(), GoblinError> {
        let store_hash = StorePreimage::<T> {
            trader,
            token_address,
        }
        .hash();

        let mut store = store_hash.load();
        let atoms_free_delta = UnsidedDeltaAtoms::try_from(store.atoms_free)?
            + self.net_delta()
            + global_transfer.net_delta()?;

        let atoms_locked_delta = UnsidedDeltaAtoms::try_from(store.atoms_locked)? - self.make;

        // Return error if free atoms > 0 or if we overflow
        store.atoms_free = UnsidedAtoms::try_from(atoms_free_delta)?;
        store.atoms_locked = UnsidedAtoms::try_from(atoms_locked_delta)?;
        store_hash.store(&store);

        let net_deposit = self.deposit.into() + global_transfer.net_delta()?;

        let Some(update_enum) = UpdateEnum::from_delta(net_deposit) else {
            return Ok(());
        };

        let deposit = net_deposit.abs();

        // match update_enum {
        //     UpdateEnum::Increase => {
        //         TransferDeposit::<Increase>::new(deposit, &token_address, &trader)
        //             .dispatch(decimals)?
        //     }
        //     UpdateEnum::Decrease => {
        //         TransferDeposit::<Decrease>::new(deposit, &token_address, &trader)
        //             .dispatch(decimals)?
        //     }
        // }

        // Handling special ETH case
        //
        // - GlobalTransfers type. It is stub for ERC20. It has net_delta()
        //
        // - Transfer in / out: convert eth_transfers.eth_out_due to delta and add to delta.deposit,
        // find update enum and perform increase / decrease accordingly. ETH deposit branch will be unreachable.
        //
        // - TransferDeposit: add TokenMarker generic parameter to get rid of token address in ETH branch.
        // Call it on ETH branch with decimals = 18. It will get monomorphized.
        Ok(())
    }
}
