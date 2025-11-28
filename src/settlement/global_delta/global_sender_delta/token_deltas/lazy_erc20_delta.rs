use core::mem::MaybeUninit;

use crate::{
    markets::LotSizePair,
    quantities::DeltaAtoms,
    settlement::{
        global_delta::{CommonDelta, ERC20Delta},
        local_delta::{LocalSenderDelta, TakerDelta},
    },
    types::{Base, LegMarker, PairAccessor, Quote},
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
    pub fn apply_local_update<In>(
        &mut self,
        lot_size_pair: &LotSizePair,
        local_sender_delta: &LocalSenderDelta,
        deposit_amount: DeltaAtoms,
    ) -> Option<()>
    where
        In: LegMarker
            + PairAccessor<
                <Base as LegMarker>::LotsPerUnit,
                <Quote as LegMarker>::LotsPerUnit,
                Result = In::LotsPerUnit,
            > + PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In>>,
        In::Opposite:
            PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In::Opposite>>,
    {
        if self.init {
            let delta = unsafe { self.inner.assume_init_mut() };
            delta
                .common_delta
                .apply_local_update::<In>(local_sender_delta, lot_size_pair)
        } else {
            self.init = true;

            // Checked addition on empty is wasteful
            // TODO fix
            let mut common_delta = CommonDelta::default();
            common_delta.apply_local_update::<In>(local_sender_delta, lot_size_pair)?;
            self.inner.write(ERC20Delta {
                deposit_due: deposit_amount,
                common_delta,
            });

            return Some(());
        }
    }
}
