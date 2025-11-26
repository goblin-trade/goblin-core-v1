use core::mem::MaybeUninit;

use crate::{
    markets::LotSizePair,
    quantities::DeltaAtoms,
    settlement::{
        global_delta::{CommonDelta, ERC20Delta},
        local_delta::LocalSenderDelta,
    },
    types::LegMarker,
};

/// Lazily-initialized ERC20 delta accumulator.
///
/// Wraps `ERC20Delta` in `MaybeUninit` with an initialization flag to avoid
/// unnecessary zero-fills. The delta is only initialized on first use.
#[derive(Clone, Copy)]
pub struct LazyERC20Delta {
    /// Whether the delta has been initialized
    pub init: bool,

    /// ERC20Delta wrapped in MaybeUninit
    pub inner: MaybeUninit<ERC20Delta>,
}

impl Default for LazyERC20Delta {
    fn default() -> Self {
        Self {
            init: false,
            inner: MaybeUninit::uninit(),
        }
    }
}

impl LazyERC20Delta {
    pub fn apply_local_update<In: LegMarker>(
        &mut self,
        local_sender_delta: &LocalSenderDelta,
        lot_size_pair: &LotSizePair,
        deposit_amount: DeltaAtoms,
    ) {
        if self.init {
            let delta = unsafe { self.inner.assume_init_mut() };
            delta
                .common_delta
                .apply_local_update::<In>(local_sender_delta, lot_size_pair); // TODO pass here
        } else {
            self.init = true;
            self.inner.write(ERC20Delta {
                deposit_due: deposit_amount,
                common_delta: CommonDelta::default(),
            });
        }
    }

    /// Accumulate a deposit amount to the delta.
    ///
    /// Initializes the delta on first non-zero deposit. The actual deposit
    /// occurs during the settlement phase.
    pub fn deposit(&mut self, deposit_amount: DeltaAtoms) -> Option<()> {
        if deposit_amount == DeltaAtoms::ZERO {
            return Some(());
        }

        if self.init {
            let delta = unsafe { self.inner.assume_init_mut() };
            delta.deposit_due = delta.deposit_due.checked_add(deposit_amount)?;
        } else {
            self.init = true;
            self.inner.write(ERC20Delta {
                deposit_due: deposit_amount,
                common_delta: CommonDelta::default(),
            });
        }
        Some(())
    }
}
