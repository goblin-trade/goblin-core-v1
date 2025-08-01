use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    instructions::take::self_trade_behavior::SelfTradeBehavior,
    markets::MarketIndex,
    quantities::{BaseLots, QuoteLots, Ticks},
    require,
    types::{OrderExpiry, Side},
};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled. This is ensured by the condition
///
/// num_base_lots == min_base_lots_to_fill, or
/// num_quote_lots == min_quote_lots_to_fill
///
#[repr(C)]
pub struct TakeHeader {
    /// The market to trade on
    pub market_index: MarketIndex,

    /// 4 bits represent flags- side, self trade behavior and expiry type (block number or block timestamp based)
    /// 28 bits represent the expiry value itself
    pub flags_and_expiry: u32,

    /// The order size, i.e. number of base lots to fill.
    /// One of num_base_lots and num_quote_lots must be zero and and the other non-zero.
    pub num_base_lots: BaseLots,

    /// The order size, i.e. number of quote lots to fill.
    /// One of num_base_lots and num_quote_lots must be zero and and the other non-zero.
    pub num_quote_lots: QuoteLots,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    /// Atleast one of min_base_lots_to_fill and min_quote_lots_to_fill must be zero.
    pub min_base_lots_to_fill: BaseLots,

    /// The minimum number of quote lots to fill, otherwise the order will be invalidated.
    /// Atleast one of min_base_lots_to_fill and min_quote_lots_to_fill must be zero.
    pub min_quote_lots_to_fill: QuoteLots,

    /// The worst price to be matched against. Stop after this price is crossed.
    pub price_limit: Ticks,

    /// Max number of orders to match against. Pass u8::MAX for max matching.
    pub match_limit: u8,
}

impl TakeHeader {
    const BYTE_SIZE: usize = core::mem::size_of::<TakeHeader>();

    pub fn decode<'a>(
        payload: &'a ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<&'a Self, GoblinError> {
        require!(
            len >= *offset + Self::BYTE_SIZE,
            GoblinError::InvalidPayload
        );
        let header = payload.decode_ref::<TakeHeader>(*offset);
        *offset += Self::BYTE_SIZE;

        require!(header.valid(), GoblinError::InvalidTakeArgs);
        Ok(header)
    }

    pub fn side(&self) -> Side {
        Side::from(self.flags_and_expiry & 0b1 != 0)
    }

    pub fn self_trade_behavior(&self) -> Result<SelfTradeBehavior, GoblinError> {
        SelfTradeBehavior::try_from((self.flags_and_expiry & 0b110) as u8)
    }

    pub fn order_expiry(&self) -> OrderExpiry {
        OrderExpiry::new(
            self.flags_and_expiry & 0b1000 != 0,
            self.flags_and_expiry >> 4,
        )
    }

    fn valid(&self) -> bool {
        // At price zero, bidding one quote lot will give undefined base lots
        (self.side() == Side::Bid && self.price_limit > Ticks::ZERO)
            // Order size must be either in base lots or quote lots
            && (self.num_base_lots == BaseLots::ZERO && self.num_quote_lots > QuoteLots::ZERO
                || self.num_base_lots > BaseLots::ZERO && self.num_quote_lots == QuoteLots::ZERO)
    }
}
