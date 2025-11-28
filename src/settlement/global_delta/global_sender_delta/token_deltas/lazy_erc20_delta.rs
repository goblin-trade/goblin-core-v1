use core::mem::MaybeUninit;

use crate::{
    quantities::DeltaAtoms,
    settlement::global_delta::{CommonDelta, ERC20Delta, GlobalUpdate},
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
    pub fn apply_global_update<In: LegMarker>(
        &mut self,
        global_update: &GlobalUpdate<In>,
        deposit_amount: DeltaAtoms,
    ) -> Option<()> {
        if self.init {
            let delta = unsafe { self.inner.assume_init_mut() };
            delta.common_delta.add_global_update(&global_update)
        } else {
            self.init = true;

            let common_delta = CommonDelta::new::<In>(&global_update);
            self.inner.write(ERC20Delta {
                deposit_due: deposit_amount,
                common_delta,
            });

            return Some(());
        }
    }
}
