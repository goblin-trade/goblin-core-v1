use crate::{
    axis::leg::leg_matcher::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode, VariableDecode},
    instructions::TakeHeader,
    quantities::Position,
    require,
};

// impl<'a, In: LegMatcher> VariableDecode<'a> for TakeHeader<In> {}

// // TODO change to VariableDecode
// impl<'a, In: LegMatcher> FixedDecode<'a> for TakeHeader<In> {
//     const ENCODED_SIZE: usize = 8;

//     fn raw_fixed_decode(ctx: &'a DecodeCtx) -> Self {
//         // 2 bits for flags and rest 62 bits for num_lots
//         let flags_and_num_lots_raw = u64::raw_fixed_decode(ctx);

//         let read_min_lots_to_fill = flags_and_num_lots_raw & 0b01 != 0;
//         let read_limit = flags_and_num_lots_raw & 0b10 != 0;

//         let num_lots = In::Lots::from(flags_and_num_lots_raw >> 2);

//         let min_lots_to_fill = In::Lots::from(match read_min_lots_to_fill {
//             true => u64::try_decode(ctx)?,
//             false => 0,
//         });

//         let limit = match read_limit {
//             true => Position::new(u64::try_decode(ctx)?),
//             false => In::DEFAULT_PRICE_LIMIT,
//         };

//         Self {
//             num_lots,
//             min_lots_to_fill,
//             limit,
//         }
//     }

//     fn validate(&self) -> Result<(), GoblinError> {
//         require!(
//             self.num_lots > In::Lots::default() && self.limit > Position::ZERO,
//             GoblinError::InvalidTakeArgs
//         );
//         Ok(())
//     }
// }
