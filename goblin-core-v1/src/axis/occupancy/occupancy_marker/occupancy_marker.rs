use crate::{
    axis::market::{market_spec::MarketSpec, Writables},
    goblin_error::GoblinError,
    instructions::MakeReadables,
    quantities::BaseLots,
    state::{
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::preimage::RestingOrderPreimage,
        SlotKey,
    },
    types::Address,
};

pub trait OccupancyMarker {
    type MakeEnum: Clone + Copy;

    // vacant = open (LegMatcher = Base / Quote)
    // occupied = update (UpdateMarker = Increase / Decrease)
    fn make<MS: MarketSpec>(
        make_readables: &MakeReadables<MS>,
        inner_enum_raw: bool,
        writables: &mut Writables,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError>;

    fn increase_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>;

    fn decrease_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>;
}
