use crate::{
    axis::{
        leg::{Base, Pair},
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    settlement::{global_delta::GlobalDelta, local_delta::LocalDelta},
    types::StoreReader,
};

/// Global static mut Delta, initially zero filled.
///
/// `static mut` allows us to take advantage of the fact that lienar memory is zero filled.
/// We get an empty starting buffer without the cost of zeroing.
static mut DELTA: Delta = Delta::zero();

pub struct Delta {
    pub global: GlobalDelta,
    pub local: LocalDelta,
}

impl Delta {
    pub const fn zero() -> Self {
        Self {
            global: GlobalDelta::zero(),
            local: LocalDelta::zero(),
        }
    }

    pub fn get_static() -> &'static mut Self {
        unsafe { &mut DELTA }
    }

    pub fn commit_local_delta<M, B, Q>(
        &mut self,
        token_index_pair: Pair<B::TokenIndex, Q::TokenIndex>,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        // Steps
        // 1. Deposits
        // 2. msg_sender
        // 3. Makers

        // Deposit goes into global_delta.global_sender_delta
        // Deposit only for ERC20 token. ETH deposit is a stub

        // Reaching deposit store
        // 1. Use B and Q to lookup in GlobalSenderDelta triple. Either by adding a function
        // on Tokenmarker or defining a common trait

        // let gg =

        // let local_deposit = self.local.deposits;
        // let base_deposit = Base::get(&self.local.deposits);

        // let global_base_deposit = B::get_leg_mut(&mut self.global.global_sender_delta);

        // // Reset local delta for reuse
        // self.local.deposits.reset::<B, Q>();
        Ok(())
    }
}
