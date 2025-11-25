use core::mem::MaybeUninit;

use crate::{
    quantities::DeltaAtoms,
    settlement::global_delta::{CommonDelta, ERC20Delta},
};

#[derive(Clone, Copy)]
pub struct ERC20DeltaMaybe {
    /// Whether the delta is initialized
    pub init: bool,

    /// ERC20Delta wrapped in MaybeUninit
    pub inner: MaybeUninit<ERC20Delta>,
}

impl Default for ERC20DeltaMaybe {
    fn default() -> Self {
        Self {
            init: false,
            inner: MaybeUninit::uninit(),
        }
    }
}

impl ERC20DeltaMaybe {
    /// Add a deposit amount to the delta accumulator.
    /// The actualy deposit happens in settlement phase.
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
