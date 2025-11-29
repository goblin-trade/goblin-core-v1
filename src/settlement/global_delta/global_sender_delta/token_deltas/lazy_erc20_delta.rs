use crate::{
    quantities::DeltaAtoms,
    settlement::global_delta::{CommonDelta, ERC20Delta, GlobalUpdate, Lazy},
    types::LegMarker,
};

/// Lazily-initialized ERC20 delta accumulator.
impl Lazy<ERC20Delta> {
    pub fn apply_global_update<In: LegMarker>(
        &mut self,
        global_update: &GlobalUpdate<In>,
        deposit_amount: DeltaAtoms,
    ) -> Option<()> {
        self.update(
            // init_fn
            || ERC20Delta {
                deposit_due: deposit_amount,
                common_delta: CommonDelta::new::<In>(global_update),
            },
            // update_fn
            |delta| delta.common_delta.add_global_update(global_update),
        );

        Some(())
    }
}
