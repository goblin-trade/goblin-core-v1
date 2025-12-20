use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    token::TokenMarker,
    types::Pair,
};

/// Tokens to be deposited for a market as read from args
pub type DepositPair<B: TokenMarker, Q: TokenMarker> = Pair<B::Deposit, Q::Deposit>;

// epoche for now

// impl<B, Q> Decodable<DepositPair<B, Q>> for DepositPair<B, Q>
// where
//     B: TokenMarker + Decodable<B::Deposit>,
//     Q: TokenMarker + Decodable<Q::Deposit>,
// {
//     fn decode(
//         args: &ArgsBuffer,
//         offset: &mut usize,
//         len: usize,
//     ) -> Result<DepositPair<B, Q>, GoblinError> {
//         let base = B::decode(args, offset, len)?;
//         let quote = Q::decode(args, offset, len)?;

//         Ok(DepositPair::<B, Q>::new(base, quote))
//     }
// }
