use crate::{
    axis::{leg::SamePair, market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::region::make_region::MakeRegion,
    quantities::{Position, SafePosition, POS_0},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};

impl Bitmap<POS_0> {
    pub fn conditional_read<M, B, Q>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        last_positions: &SamePair<Position>,
        safe_position: SafePosition<POS_0>,
    ) -> (SlotKey<BitmapPreimage<M, B, Q, POS_0>>, Self)
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let key = BitmapPreimage::<M, B, Q, POS_0> {
            market_key,
            safe_position,
        }
        .hash();

        let region = MakeRegion::new(last_positions, safe_position.position());

        let bitmap = if region == MakeRegion::Spread {
            Bitmap::<POS_0>::default()
        } else {
            let outer_bitmap = key.load();
            if outer_bitmap.is_closed() {
                Bitmap::<POS_0>::default()
            } else {
                outer_bitmap
            }
        };

        (key, bitmap)
    }
}
