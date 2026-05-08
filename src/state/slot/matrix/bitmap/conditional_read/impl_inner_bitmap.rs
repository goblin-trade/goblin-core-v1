use crate::{
    axis::{leg::SamePair, market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::region::make_region::MakeRegion,
    quantities::{OuterPos, Position, INNER_POS, OUTER_POS},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};

impl Bitmap<INNER_POS> {
    pub fn conditional_read<M, B, Q>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        last_positions: &SamePair<Position>,
        position: Position,
        outer_bitmap_state: &Bitmap<OUTER_POS>,
        outer_pos: OuterPos,
    ) -> (SlotKey<BitmapPreimage<M, B, Q, INNER_POS>>, Self)
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let key = BitmapPreimage::<M, B, Q, INNER_POS> {
            market_key,
            position,
        }
        .hash();

        let region = MakeRegion::new(last_positions, position);

        let bitmap = if region == MakeRegion::Spread || !outer_bitmap_state.index_active(outer_pos)
        {
            Bitmap::<INNER_POS>::default()
        } else {
            key.load()
        };

        (key, bitmap)
    }
}
