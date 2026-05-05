use crate::{
    axis::{leg::SamePair, market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::region::make_region::MakeRegion,
    quantities::{Position, OUTER_POS_V2},
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        MarketPreimage, Preimage, SlotKey,
    },
};

impl BitmapV2<OUTER_POS_V2> {
    pub fn conditional_read<M, B, Q>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        last_positions: &SamePair<Position>,
        position: Position,
    ) -> (SlotKey<BitmapPreimageV2<M, B, Q, OUTER_POS_V2>>, Self)
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let key = BitmapPreimageV2::<M, B, Q, OUTER_POS_V2> {
            market_key,
            position,
        }
        .hash();

        let region = MakeRegion::new(last_positions, position);

        let bitmap = if region == MakeRegion::Spread {
            BitmapV2::<OUTER_POS_V2>::default()
        } else {
            let outer_bitmap = key.load();
            if outer_bitmap.is_closed() {
                BitmapV2::<OUTER_POS_V2>::default()
            } else {
                outer_bitmap
            }
        };

        (key, bitmap)
    }
}
