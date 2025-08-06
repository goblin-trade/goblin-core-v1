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
    // TODO redesign- a basic order will just have market id, side, size, slippage
    //
    // Advanced fields- price limit, match limit, self trade behavior, order expiry
    // Allocate 4 bits for this, plus 1 bit for side.
    // The first byte can be used
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

pub struct TakeHeaderV2 {
    pub market_index: MarketIndex,
    pub side: Side,
    num_lots: u64,
    min_lots_to_fill: u64,

    // Rest optional
    pub price_limit: Ticks,
    pub match_limit: u8,
    pub self_trade_behavior: SelfTradeBehavior,
    pub expiry: Option<OrderExpiry>,
}

struct TakeHeaderFlags {
    pub side: Side,
    pub read_price_limit: bool,
    pub read_match_limit: bool,
    pub read_self_trade_behavior: bool,
    pub read_expiry: bool,
    pub is_block_number_expiry: bool,
}

impl TakeHeaderFlags {
    fn new(flags: u8) -> Self {
        Self {
            side: Side::from(flags & 0b1 == 1),
            read_price_limit: flags & 0b10 == 1,
            read_match_limit: flags & 0b100 == 1,
            read_self_trade_behavior: flags & 0b1000 == 1,
            read_expiry: flags & 0b1_0000 == 1,
            is_block_number_expiry: flags & 0b10_0000 == 1,
        }
    }
}

impl TakeHeaderV2 {
    // Fixed min size for
    // * flags: 1
    // * market_index: 1
    // * num_lots: 8
    // * min_lots_to_fill: 8
    const MIN_SIZE: usize = 1 + 1 + 8 + 8;

    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        crate::require!(
            len >= *offset + Self::MIN_SIZE,
            crate::goblin_error::GoblinError::InvalidPayload
        );

        // Fixed fields
        // We should perform a single size check instead of doing 3
        let flags = payload.decode_unchecked::<u8>(*offset);
        let TakeHeaderFlags {
            side,
            read_price_limit,
            read_match_limit,
            read_self_trade_behavior,
            read_expiry,
            is_block_number_expiry,
        } = TakeHeaderFlags::new(flags);

        let market_index = MarketIndex(payload.decode_unchecked::<u8>(*offset));
        let num_lots = payload.decode_unchecked::<u64>(*offset);
        let min_lots_to_fill = payload.decode_unchecked::<u64>(*offset);

        // Decode optional fields

        let price_limit = match read_price_limit {
            true => Ticks(payload.decode::<u32>(offset, len)?),
            false => match side {
                Side::Bid => Ticks::MAX,
                Side::Ask => Ticks::ZERO,
            },
        };

        let match_limit = match read_match_limit {
            true => payload.decode::<u8>(offset, len)?,
            false => u8::MAX,
        };

        let self_trade_behavior = match read_self_trade_behavior {
            true => SelfTradeBehavior::try_from(payload.decode::<u8>(offset, len)?)?,
            false => SelfTradeBehavior::CancelProvide,
        };

        let expiry = match read_expiry {
            true => Some(OrderExpiry::new(
                is_block_number_expiry,
                payload.decode::<u32>(offset, len)?,
            )),
            false => None,
        };

        Ok(Self {
            market_index,
            side,
            num_lots,
            min_lots_to_fill,
            price_limit,
            match_limit,
            self_trade_behavior,
            expiry,
        })
    }
}
