use crate::error::GoblinSdkError;

/// Token variants supported by goblin-core-v1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TokenKind {
    /// Native ETH (index 0)
    ETH = 0,
    /// Hardcoded ERC20 with pre-configured contract address (index 1)
    HardcodedERC20 = 1,
    /// Custom ERC20 registered dynamically in global args (index 2)
    CustomERC20 = 2,
}

impl TokenKind {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::ETH => "ETH",
            Self::HardcodedERC20 => "HardcodedERC20",
            Self::CustomERC20 => "CustomERC20",
        }
    }

    pub const fn is_eth(&self) -> bool {
        matches!(self, Self::ETH)
    }

    pub const fn is_erc20(&self) -> bool {
        !self.is_eth()
    }
}

/// Market variants supported by goblin-core-v1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MarketKind {
    /// Hardcoded market from fixed static list (index 0)
    Hardcoded = 0,
    /// Dynamic/custom market specified inline (index 1)
    Dynamic = 1,
}

/// The 11 legal market specifications in goblin-core-v1, ordered exactly as processed by the smart contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum MarketSpecIndex {
    /// #0: Hardcoded market (ETH, HardcodedERC20)
    HardcodedEthHardcodedERC20 = 0,
    /// #1: Hardcoded market (HardcodedERC20, ETH)
    HardcodedHardcodedERC20Eth = 1,
    /// #2: Hardcoded market (HardcodedERC20, HardcodedERC20)
    HardcodedHardcodedERC20HardcodedERC20 = 2,

    /// #3: Dynamic market (ETH, HardcodedERC20)
    DynamicEthHardcodedERC20 = 3,
    /// #4: Dynamic market (ETH, CustomERC20)
    DynamicEthCustomERC20 = 4,
    /// #5: Dynamic market (HardcodedERC20, ETH)
    DynamicHardcodedERC20Eth = 5,
    /// #6: Dynamic market (HardcodedERC20, HardcodedERC20)
    DynamicHardcodedERC20HardcodedERC20 = 6,
    /// #7: Dynamic market (HardcodedERC20, CustomERC20)
    DynamicHardcodedERC20CustomERC20 = 7,
    /// #8: Dynamic market (CustomERC20, ETH)
    DynamicCustomERC20Eth = 8,
    /// #9: Dynamic market (CustomERC20, HardcodedERC20)
    DynamicCustomERC20HardcodedERC20 = 9,
    /// #10: Dynamic market (CustomERC20, CustomERC20)
    DynamicCustomERC20CustomERC20 = 10,
}

impl MarketSpecIndex {
    pub const ALL: [MarketSpecIndex; 11] = [
        MarketSpecIndex::HardcodedEthHardcodedERC20,
        MarketSpecIndex::HardcodedHardcodedERC20Eth,
        MarketSpecIndex::HardcodedHardcodedERC20HardcodedERC20,
        MarketSpecIndex::DynamicEthHardcodedERC20,
        MarketSpecIndex::DynamicEthCustomERC20,
        MarketSpecIndex::DynamicHardcodedERC20Eth,
        MarketSpecIndex::DynamicHardcodedERC20HardcodedERC20,
        MarketSpecIndex::DynamicHardcodedERC20CustomERC20,
        MarketSpecIndex::DynamicCustomERC20Eth,
        MarketSpecIndex::DynamicCustomERC20HardcodedERC20,
        MarketSpecIndex::DynamicCustomERC20CustomERC20,
    ];

    pub const fn index(&self) -> usize {
        *self as usize
    }

    pub const fn is_dynamic(&self) -> bool {
        self.index() >= 3
    }

    pub const fn is_hardcoded(&self) -> bool {
        self.index() < 3
    }

    pub const fn market_kind(&self) -> MarketKind {
        if self.is_dynamic() {
            MarketKind::Dynamic
        } else {
            MarketKind::Hardcoded
        }
    }

    pub const fn base_token(&self) -> TokenKind {
        match self {
            Self::HardcodedEthHardcodedERC20
            | Self::DynamicEthHardcodedERC20
            | Self::DynamicEthCustomERC20 => TokenKind::ETH,
            Self::HardcodedHardcodedERC20Eth
            | Self::HardcodedHardcodedERC20HardcodedERC20
            | Self::DynamicHardcodedERC20Eth
            | Self::DynamicHardcodedERC20HardcodedERC20
            | Self::DynamicHardcodedERC20CustomERC20 => TokenKind::HardcodedERC20,
            Self::DynamicCustomERC20Eth
            | Self::DynamicCustomERC20HardcodedERC20
            | Self::DynamicCustomERC20CustomERC20 => TokenKind::CustomERC20,
        }
    }

    pub const fn quote_token(&self) -> TokenKind {
        match self {
            Self::HardcodedHardcodedERC20Eth
            | Self::DynamicHardcodedERC20Eth
            | Self::DynamicCustomERC20Eth => TokenKind::ETH,
            Self::HardcodedEthHardcodedERC20
            | Self::HardcodedHardcodedERC20HardcodedERC20
            | Self::DynamicEthHardcodedERC20
            | Self::DynamicHardcodedERC20HardcodedERC20
            | Self::DynamicCustomERC20HardcodedERC20 => TokenKind::HardcodedERC20,
            Self::DynamicEthCustomERC20
            | Self::DynamicHardcodedERC20CustomERC20
            | Self::DynamicCustomERC20CustomERC20 => TokenKind::CustomERC20,
        }
    }

    /// Try to determine the MarketSpecIndex from market kind and base/quote token kinds.
    pub const fn from_tokens(
        market_kind: MarketKind,
        base: TokenKind,
        quote: TokenKind,
    ) -> Option<MarketSpecIndex> {
        match (market_kind, base, quote) {
            (MarketKind::Hardcoded, TokenKind::ETH, TokenKind::HardcodedERC20) => {
                Some(MarketSpecIndex::HardcodedEthHardcodedERC20)
            }
            (MarketKind::Hardcoded, TokenKind::HardcodedERC20, TokenKind::ETH) => {
                Some(MarketSpecIndex::HardcodedHardcodedERC20Eth)
            }
            (MarketKind::Hardcoded, TokenKind::HardcodedERC20, TokenKind::HardcodedERC20) => {
                Some(MarketSpecIndex::HardcodedHardcodedERC20HardcodedERC20)
            }
            (MarketKind::Dynamic, TokenKind::ETH, TokenKind::HardcodedERC20) => {
                Some(MarketSpecIndex::DynamicEthHardcodedERC20)
            }
            (MarketKind::Dynamic, TokenKind::ETH, TokenKind::CustomERC20) => {
                Some(MarketSpecIndex::DynamicEthCustomERC20)
            }
            (MarketKind::Dynamic, TokenKind::HardcodedERC20, TokenKind::ETH) => {
                Some(MarketSpecIndex::DynamicHardcodedERC20Eth)
            }
            (MarketKind::Dynamic, TokenKind::HardcodedERC20, TokenKind::HardcodedERC20) => {
                Some(MarketSpecIndex::DynamicHardcodedERC20HardcodedERC20)
            }
            (MarketKind::Dynamic, TokenKind::HardcodedERC20, TokenKind::CustomERC20) => {
                Some(MarketSpecIndex::DynamicHardcodedERC20CustomERC20)
            }
            (MarketKind::Dynamic, TokenKind::CustomERC20, TokenKind::ETH) => {
                Some(MarketSpecIndex::DynamicCustomERC20Eth)
            }
            (MarketKind::Dynamic, TokenKind::CustomERC20, TokenKind::HardcodedERC20) => {
                Some(MarketSpecIndex::DynamicCustomERC20HardcodedERC20)
            }
            (MarketKind::Dynamic, TokenKind::CustomERC20, TokenKind::CustomERC20) => {
                Some(MarketSpecIndex::DynamicCustomERC20CustomERC20)
            }
            _ => None,
        }
    }
}

/// Leg side of a trade (Base or Quote)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LegSide {
    Base = 0,
    Quote = 1,
}

/// Make order operation types: Open, Increase, Decrease, Close
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MakeAction {
    /// Open a new resting order on a vacant position
    Open { side: LegSide, base_lots: u64 },
    /// Increase base lots on an existing occupied resting order
    Increase { base_lots: u64 },
    /// Decrease base lots on an existing occupied resting order
    Decrease { base_lots: u64 },
    /// Close an existing resting order by reducing base lots
    Close { base_lots: u64 },
}

impl MakeAction {
    pub const fn base_lots(&self) -> u64 {
        match *self {
            Self::Open { base_lots, .. }
            | Self::Increase { base_lots }
            | Self::Decrease { base_lots }
            | Self::Close { base_lots } => base_lots,
        }
    }

    /// Returns (occupancy_bit, inner_enum_raw_bit) for wire encoding
    pub const fn encode_bits(&self) -> (bool, bool) {
        match *self {
            // Vacant (occupancy = false): inner_enum_raw is LegEnum (Base = false, Quote = true)
            Self::Open { side, .. } => (false, matches!(side, LegSide::Quote)),
            // Occupied (occupancy = true): inner_enum_raw is UpdateEnum (Increase = false, Decrease = true)
            Self::Increase { .. } => (true, false),
            Self::Decrease { .. } | Self::Close { .. } => (true, true),
        }
    }
}

/// A make order positioned at a 64-bit position / tick
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionedMakeOrder {
    pub position: u64,
    pub action: MakeAction,
}

impl PositionedMakeOrder {
    pub const fn new(position: u64, action: MakeAction) -> Self {
        Self { position, action }
    }

    pub const fn open(position: u64, side: LegSide, base_lots: u64) -> Self {
        Self {
            position,
            action: MakeAction::Open { side, base_lots },
        }
    }

    pub const fn increase(position: u64, base_lots: u64) -> Self {
        Self {
            position,
            action: MakeAction::Increase { base_lots },
        }
    }

    pub const fn decrease(position: u64, base_lots: u64) -> Self {
        Self {
            position,
            action: MakeAction::Decrease { base_lots },
        }
    }

    pub const fn close(position: u64, base_lots: u64) -> Self {
        Self {
            position,
            action: MakeAction::Close { base_lots },
        }
    }
}

/// Take order (Immediate-or-Cancel / Market Order)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TakeOrder {
    /// Number of lots to fill (must be > 0)
    pub num_lots: u64,
    /// Minimum lots to fill (for Fill-or-Kill or slippage control)
    pub min_lots_to_fill: Option<u64>,
    /// Worst position limit to match against
    pub limit: Option<u64>,
}

impl TakeOrder {
    pub const fn new(num_lots: u64) -> Self {
        Self {
            num_lots,
            min_lots_to_fill: None,
            limit: None,
        }
    }

    pub const fn with_min_lots(mut self, min_lots: u64) -> Self {
        self.min_lots_to_fill = Some(min_lots);
        self
    }

    pub const fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Deposit amounts for a market (in local market namespace)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MarketDeposits {
    pub base_deposit: i64,
    pub quote_deposit: i64,
}

impl MarketDeposits {
    pub const fn new(base_deposit: i64, quote_deposit: i64) -> Self {
        Self {
            base_deposit,
            quote_deposit,
        }
    }

    pub const fn is_zero(&self) -> bool {
        self.base_deposit == 0 && self.quote_deposit == 0
    }
}

/// Locator identifying a market
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketLocator {
    /// Hardcoded market specified by index in static list
    Hardcoded { market_index: u8 },
    /// Dynamic market specified by token indices, lot sizes, and tick size
    Dynamic {
        base_token_index: u8,
        quote_token_index: u8,
        base_lot_size: u64,
        quote_lot_size: u64,
        tick_size: u64,
    },
}

/// Represents all instructions to execute for a specific market instance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketCall {
    pub spec: MarketSpecIndex,
    pub locator: MarketLocator,
    pub deposits: Option<MarketDeposits>,
    pub base_take: Option<TakeOrder>,
    pub quote_take: Option<TakeOrder>,
    pub makes: Vec<PositionedMakeOrder>,
}

impl MarketCall {
    pub fn new_hardcoded(spec: MarketSpecIndex, market_index: u8) -> Result<Self, GoblinSdkError> {
        if spec.is_dynamic() {
            return Err(GoblinSdkError::InvalidTokenPair {
                market: "Hardcoded",
                base: spec.base_token().name(),
                quote: spec.quote_token().name(),
            });
        }

        Ok(Self {
            spec,
            locator: MarketLocator::Hardcoded { market_index },
            deposits: None,
            base_take: None,
            quote_take: None,
            makes: Vec::new(),
        })
    }

    pub fn new_dynamic(
        base_kind: TokenKind,
        base_token_index: u8,
        quote_kind: TokenKind,
        quote_token_index: u8,
        base_lot_size: u64,
        quote_lot_size: u64,
        tick_size: u64,
    ) -> Result<Self, GoblinSdkError> {
        let spec = MarketSpecIndex::from_tokens(MarketKind::Dynamic, base_kind, quote_kind).ok_or(
            GoblinSdkError::InvalidTokenPair {
                market: "Dynamic",
                base: base_kind.name(),
                quote: quote_kind.name(),
            },
        )?;

        // Validate lot sizes (must divide 1_000_000 ATOMS_PER_UNIT)
        const ATOMS_PER_UNIT: u64 = 1_000_000;
        if base_lot_size == 0 || ATOMS_PER_UNIT % base_lot_size != 0 {
            return Err(GoblinSdkError::InvalidLotSize(base_lot_size));
        }
        if quote_lot_size == 0 || ATOMS_PER_UNIT % quote_lot_size != 0 {
            return Err(GoblinSdkError::InvalidLotSize(quote_lot_size));
        }

        Ok(Self {
            spec,
            locator: MarketLocator::Dynamic {
                base_token_index,
                quote_token_index,
                base_lot_size,
                quote_lot_size,
                tick_size,
            },
            deposits: None,
            base_take: None,
            quote_take: None,
            makes: Vec::new(),
        })
    }
}
