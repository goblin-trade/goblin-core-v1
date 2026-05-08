use crate::{
    axis::{leg::SamePair, market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::region::make_region::MakeRegion,
    quantities::{Position, OUTER_POS},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};

impl Bitmap<OUTER_POS> {
    pub fn conditional_read<M, B, Q>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        last_positions: &SamePair<Position>,
        position: Position,
    ) -> (SlotKey<BitmapPreimage<M, B, Q, OUTER_POS>>, Self)
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let key = BitmapPreimage::<M, B, Q, OUTER_POS> {
            market_key,
            position,
        }
        .hash();

        let region = MakeRegion::new(last_positions, position);

        let bitmap = if region == MakeRegion::Spread {
            Bitmap::<OUTER_POS>::default()
        } else {
            let outer_bitmap = key.load();
            if outer_bitmap.is_closed() {
                Bitmap::<OUTER_POS>::default()
            } else {
                outer_bitmap
            }
        };

        (key, bitmap)
    }
}
