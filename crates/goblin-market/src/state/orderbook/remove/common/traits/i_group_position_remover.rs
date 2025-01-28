use crate::{
    quantities::Ticks,
    state::{bitmap_group::BitmapGroup, ArbContext, OuterIndex, Side},
};

pub trait IGroupPositionRemover {
    /// Load bitmap group for the given outer index
    ///
    /// # Arguments
    ///
    /// * `ctx` - Context to read from slot
    /// * `outer_index` - Load bitmap group for this outer index
    fn load_outer_index(&mut self, ctx: &ArbContext, outer_index: OuterIndex);

    /// Get the current bitmap group
    fn get_bitmap_group(&self) -> BitmapGroup;

    /// Externally set the bitmap group
    fn set_bitmap_group(&mut self, bitmap_group: BitmapGroup);

    /// Load bitmap group for the outermost outer index, ignoring the garbage bits
    ///
    /// # Arguments
    ///
    /// * `ctx` - Context to read from slot
    /// * `best_market_price` - Best market price for side
    fn load_outermost_group(&mut self, ctx: &ArbContext, best_market_price: Ticks);

    /// Write the bitmap group to slot
    fn write_to_slot(&self, ctx: &mut ArbContext, outer_index: OuterIndex);

    /// Get side for this remover
    fn side(&self) -> Side;
}
