use crate::{
    axis::{
        leg::{leg_reader::LegReader, SamePair},
        token::{
            token_deltas::TokenDeltas,
            token_index::{TokenData, TokenIndex},
            token_marker::TokenMarker,
            token_msg_transfer::TokenMsgTransfer,
        },
        update::{Decrease, Increase, UpdateEnum},
    },
    goblin_error::GoblinError,
    hostio::erc20_hostio,
    input_processor::MsgTransfers,
    quantities::{IntoAbs, UnsidedAtoms, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot},
    settlement::local_delta::LocalDelta,
    state::{Preimage, SlotState, StorePreimage},
    types::Address,
};

#[derive(Clone, Copy)]
pub struct TokenDelta<T: TokenDeltas> {
    pub deposit: T::GlobalDeposit,
    pub take: UnsidedDeltaAtoms,
    pub make: UnsidedDeltaAtoms,
}

impl<T: TokenDeltas> TokenDelta<T> {
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
        (token_index, token_data): (T::TokenIndex, TokenData<T>),
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker, // where
                        //     <T::TokenIndex as TokenIndex>::StoredDecimals:
                        //         TryFrom<<T::TokenIndex as TokenIndex>::HostioDecimals, Error = GoblinError>,
    {
        let token_address = token_data.address;
        let msg_transfer = T::get(msg_transfers);

        let store_hash = StorePreimage::<T> {
            trader: *trader,
            token_address,
        }
        .hash();

        let mut store = store_hash.load();

        if store.is_empty() {
            let hostio_decimals: <T::TokenIndex as TokenIndex>::HostioDecimals =
                token_index.get_hostio_decimals(&token_address)?;
            // .map(|gg| <T::TokenIndex as TokenIndex>::StoredDecimals::try_from(gg))?;

            let stored_decimals =
                <T::TokenIndex as TokenIndex>::StoredDecimals::try_from(hostio_decimals)
                    .map_err(|_| GoblinError::NoHostioDecimals)?;

            // spagetti code
            // Hardcoded decimals are already present. Yet we need to define a getter function
            // for hardcoded that never gets used.
            //
            // token_index.get_decimals() will perform hostio call for custom erc20
            //
            // General idea
            //
            // Try to convert hardcoded decimals into stored decimals.
            // If this fails try to convert 'hostioDecimals' into 'stored decimals'
            //
            // Define a new generic and getter function
            // store.decimals = token_data.decimals.try_into().or(token_index
            //     .get_hostio_decimals(&token_address)
            //     .map(|g| g.try_into())?)?;
        }

        let atoms_free_delta = UnsidedDeltaAtoms::try_from(store.atoms_free)?
            + self.net_delta()
            + msg_transfer.net_delta()?;

        let atoms_locked_delta = UnsidedDeltaAtoms::try_from(store.atoms_locked)? - self.make;

        // Return error if free atoms > 0 or if we overflow
        store.atoms_free = UnsidedAtoms::try_from(atoms_free_delta)?;
        store.atoms_locked = UnsidedAtoms::try_from(atoms_locked_delta)?;

        // TODO write only if values changed
        // Instead of comparing states, just check if delta is non-zero?
        store_hash.store(&store);

        let net_deposit = self.deposit.into() + msg_transfer.deposit_due()?;

        let Some(update_enum) = UpdateEnum::from_delta(net_deposit) else {
            return Ok(());
        };

        let deposit = net_deposit.abs();

        match update_enum {
            UpdateEnum::Increase => {
                T::update::<Increase>(deposit, trader, &token_address, store.decimals)
            }
            UpdateEnum::Decrease => {
                T::update::<Decrease>(deposit, trader, &token_address, store.decimals)
            }
        }
    }
}
