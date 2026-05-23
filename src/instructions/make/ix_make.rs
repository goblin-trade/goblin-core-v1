use crate::{
    axis::{
        market::{header::make_header::MakeHeader, market_marker::MarketMarker, Readables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::{make_variant::MakeVariant, MakeWritables, PosHeader},
    quantities::{SafePosition, POS_1, POS_2},
};

impl<'a> MakeWritables<'a> {
    pub fn ix_make<M, B, Q>(
        &mut self,
        ctx: &DecodeCtx,
        readables: &Readables<M, B, Q>,
        pos_1: SafePosition<POS_1>,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let MakeHeader {
            inner_pos,
            base_lots,
            make_variant,
        } = MakeHeader::try_decode(ctx)?;

        let pos_2 = SafePosition::<POS_2>::new(pos_1, inner_pos);
        let position = pos_2.into();

        let pos_header = PosHeader {
            position,
            base_lots,
        };

        match make_variant {
            MakeVariant::Update(update_enum) => self.ix_update(readables, pos_header, update_enum),
            MakeVariant::Open(leg_enum) => self.ix_open(readables, pos_header, leg_enum),
            MakeVariant::Limit(leg_enum) => self.ix_limit(readables, pos_header, leg_enum),
        }
    }
}
