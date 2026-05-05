use crate::{
    axis::{leg::SamePair, market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::region::make_region::MakeRegion,
    quantities::{OuterPosV2, Position, INNER_POS_V2, OUTER_POS_V2},
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        MarketPreimage, Preimage, SlotKey,
    },
};

impl BitmapV2<INNER_POS_V2> {
    pub fn conditional_read<M, B, Q>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        last_positions: &SamePair<Position>,
        position: Position,
        outer_bitmap_state: &BitmapV2<OUTER_POS_V2>,
        outer_pos: OuterPosV2,
    ) -> (SlotKey<BitmapPreimageV2<M, B, Q, INNER_POS_V2>>, Self)
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let key = BitmapPreimageV2::<M, B, Q, INNER_POS_V2> {
            market_key,
            position,
        }
        .hash();

        let region = MakeRegion::new(last_positions, position);

        let bitmap = if region == MakeRegion::Spread || !outer_bitmap_state.index_active(outer_pos)
        {
            BitmapV2::<INNER_POS_V2>::default()
        } else {
            key.load()
        };

        (key, bitmap)
    }
}
