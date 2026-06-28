use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, Readables, Writables},
        occupancy::occupancy_marker::OccupancyMarker,
        token::token_marker::TokenMarker,
        update::update_reader::UpdateReader,
    },
    goblin_error::GoblinError,
    instructions::{MakeReadables, PosHeader},
    quantities::{BaseLots, InnerPos, Ticks},
    state::{
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::preimage::RestingOrderPreimage,
        Preimage, SlotKey,
    },
    types::{Address, StoreReader},
};

/// Marker tracking increase or decrease in free atoms from store
pub trait UpdateMarker: Sized {
    const SIGN: i64;

    fn process_update<'a, M, B, Q, In, Oc>(
        make_readables: &MakeReadables<M, B, Q>,
        writables: &mut Writables,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
        Oc: OccupancyMarker,
        Self: UpdateReader<In>,
    {
        let Readables {
            msg_sender,
            market_readables,
        } = *make_readables.readables;

        let PosHeader {
            position,
            base_lots,
        } = make_readables.pos_header;

        let key = &mut RestingOrderPreimage {
            market_key: market_readables.market_key,
            position,
        }
        .hash();

        let updated_base_lots = Self::update_resting_order::<M, B, Q, In, Oc>(
            msg_sender,
            base_lots,
            key,
            &mut InnerBitmapUpdater {
                bitmap: inner_bitmap_state,
                pos: InnerPos::from(position),
            },
        )?;

        let base_lot_size = Base::get(&market_readables.market.lot_size_pair);
        let tick_size = market_readables.market.tick_size;
        let price = Ticks::from(position);

        writables.local_delta.make.add_make::<In, Self>(
            updated_base_lots,
            base_lot_size,
            tick_size,
            price,
        )
    }

    fn update_resting_order<'a, M, B, Q, In, Oc>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
        Oc: OccupancyMarker;
}
