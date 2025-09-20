use crate::{
    matching::MatchResult,
    types::{Base, Quote},
};

#[derive(Default)]
pub struct MarketSenderDelta {
    pub take_base_in: MatchResult<Base>,
    pub take_quote_in: MatchResult<Quote>,
    // Add limit order and cancel fields later
}
