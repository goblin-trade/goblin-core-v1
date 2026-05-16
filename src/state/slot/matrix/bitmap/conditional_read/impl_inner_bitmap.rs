use crate::{
    axis::{leg::SamePair, market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::region::make_region::MakeRegion,
    quantities::{OuterPos, Position, SafePosition, POS_0, POS_1},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};

impl Bitmap<POS_1> {
    pub fn conditional_read<M, B, Q>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        last_positions: &SamePair<Position>,
        safe_position: SafePosition<POS_1>,
        outer_bitmap_state: &Bitmap<POS_0>,
        outer_pos: OuterPos,
    ) -> (SlotKey<BitmapPreimage<M, B, Q, POS_1>>, Self)
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let key = BitmapPreimage::<M, B, Q, POS_1> {
            market_key,
            safe_position,
        }
        .hash();

        let region = MakeRegion::new(last_positions, safe_position.position());

        let bitmap = if region == MakeRegion::Spread || !outer_bitmap_state.index_active(outer_pos)
        {
            Bitmap::<POS_1>::default()
        } else {
            key.load()
        };

        (key, bitmap)
    }
}
