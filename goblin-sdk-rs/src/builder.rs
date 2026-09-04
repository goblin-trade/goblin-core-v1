use crate::{
    encoder::{GlobalPayloadConfig, GoblinEncoder},
    error::GoblinSdkError,
    types::{
        LegSide, MarketCall, MarketDeposits, MarketLocator, MarketSpecIndex, PositionedMakeOrder,
        TakeOrder, TokenKind,
    },
};

/// High-level builder for constructing calldata payloads to call goblin-core-v1.
#[derive(Debug, Clone, Default)]
pub struct GoblinCalldataBuilder {
    config: GlobalPayloadConfig,
}

impl GoblinCalldataBuilder {
    /// Create a new empty calldata builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a custom recipient for token/ETH payouts
    pub fn set_recipient(&mut self, recipient: [u8; 20]) -> &mut Self {
        self.config.custom_recipient = Some(recipient);
        self
    }

    /// Set a custom recipient from a hex string (0x-prefixed or raw 40 hex chars)
    pub fn set_recipient_hex(&mut self, recipient_hex: &str) -> Result<&mut Self, GoblinSdkError> {
        let clean_hex = recipient_hex.strip_prefix("0x").unwrap_or(recipient_hex);
        let bytes = hex::decode(clean_hex)
            .map_err(|_| GoblinSdkError::InvalidHexAddress(recipient_hex.to_string()))?;
        if bytes.len() != 20 {
            return Err(GoblinSdkError::InvalidHexAddress(recipient_hex.to_string()));
        }
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&bytes);
        self.config.custom_recipient = Some(addr);
        Ok(self)
    }

    /// Enable or disable reading msg.value by hostio
    pub fn set_msg_value(&mut self, read_msg_value: bool) -> &mut Self {
        self.config.read_msg_value = read_msg_value;
        self
    }

    /// Configure ETH withdrawal amount and whether to credit internally or transfer externally
    pub fn set_eth_withdrawal(&mut self, amount: u64, internally: bool) -> &mut Self {
        self.config.withdraw_eth_amount = amount;
        self.config.withdraw_internally = internally;
        self
    }

    /// Register a custom ERC20 token address and return its allocated 0-based custom token index
    pub fn add_custom_token(&mut self, address: [u8; 20]) -> Result<u8, GoblinSdkError> {
        if self.config.custom_tokens.len() >= 7 {
            return Err(GoblinSdkError::CustomTokenLimitExceeded(
                self.config.custom_tokens.len() + 1,
            ));
        }
        let idx = self.config.custom_tokens.len() as u8;
        self.config.custom_tokens.push(address);
        Ok(idx)
    }

    /// Register a custom ERC20 token address from hex and return its index
    pub fn add_custom_token_hex(&mut self, address_hex: &str) -> Result<u8, GoblinSdkError> {
        let clean_hex = address_hex.strip_prefix("0x").unwrap_or(address_hex);
        let bytes = hex::decode(clean_hex)
            .map_err(|_| GoblinSdkError::InvalidHexAddress(address_hex.to_string()))?;
        if bytes.len() != 20 {
            return Err(GoblinSdkError::InvalidHexAddress(address_hex.to_string()));
        }
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&bytes);
        self.add_custom_token(addr)
    }

    /// Add a pre-built market call to the transaction
    pub fn add_market(&mut self, market: MarketCall) -> &mut Self {
        self.config.markets.push(market);
        self
    }

    /// Add a market call using a closure with MarketCallBuilder
    pub fn with_market<F>(&mut self, builder_fn: F) -> Result<&mut Self, GoblinSdkError>
    where
        F: FnOnce(&mut MarketCallBuilder) -> Result<(), GoblinSdkError>,
    {
        let mut builder = MarketCallBuilder::default();
        builder_fn(&mut builder)?;
        let call = builder.build()?;
        self.add_market(call);
        Ok(self)
    }

    /// Serialize the entire calldata payload to raw bytes
    pub fn build(&self) -> Result<Vec<u8>, GoblinSdkError> {
        GoblinEncoder::encode_payload(&self.config)
    }

    /// Serialize the entire calldata payload to a hex string with "0x" prefix
    pub fn build_hex(&self) -> Result<String, GoblinSdkError> {
        let bytes = self.build()?;
        Ok(format!("0x{}", hex::encode(bytes)))
    }
}

/// Builder for constructing instructions for a single market instance
#[derive(Debug, Clone, Default)]
pub struct MarketCallBuilder {
    spec: Option<MarketSpecIndex>,
    locator: Option<MarketLocator>,
    deposits: Option<MarketDeposits>,
    base_take: Option<TakeOrder>,
    quote_take: Option<TakeOrder>,
    makes: Vec<PositionedMakeOrder>,
}

impl MarketCallBuilder {
    /// Create a new empty market call builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure as a hardcoded market
    pub fn hardcoded(
        &mut self,
        spec: MarketSpecIndex,
        market_index: u8,
    ) -> Result<&mut Self, GoblinSdkError> {
        if spec.is_dynamic() {
            return Err(GoblinSdkError::InvalidTokenPair {
                market: "Hardcoded",
                base: spec.base_token().name(),
                quote: spec.quote_token().name(),
            });
        }
        self.spec = Some(spec);
        self.locator = Some(MarketLocator::Hardcoded { market_index });
        Ok(self)
    }

    /// Configure as a dynamic market
    pub fn dynamic(
        &mut self,
        base_kind: TokenKind,
        base_token_index: u8,
        quote_kind: TokenKind,
        quote_token_index: u8,
        base_lot_size: u64,
        quote_lot_size: u64,
        tick_size: u64,
    ) -> Result<&mut Self, GoblinSdkError> {
        let spec =
            MarketSpecIndex::from_tokens(crate::types::MarketKind::Dynamic, base_kind, quote_kind)
                .ok_or(GoblinSdkError::InvalidTokenPair {
                    market: "Dynamic",
                    base: base_kind.name(),
                    quote: quote_kind.name(),
                })?;

        const ATOMS_PER_UNIT: u64 = 1_000_000;
        if base_lot_size == 0 || ATOMS_PER_UNIT % base_lot_size != 0 {
            return Err(GoblinSdkError::InvalidLotSize(base_lot_size));
        }
        if quote_lot_size == 0 || ATOMS_PER_UNIT % quote_lot_size != 0 {
            return Err(GoblinSdkError::InvalidLotSize(quote_lot_size));
        }

        self.spec = Some(spec);
        self.locator = Some(MarketLocator::Dynamic {
            base_token_index,
            quote_token_index,
            base_lot_size,
            quote_lot_size,
            tick_size,
        });
        Ok(self)
    }

    /// Set deposit amounts for base and quote tokens
    pub fn deposit(&mut self, base_deposit: i64, quote_deposit: i64) -> &mut Self {
        self.deposits = Some(MarketDeposits::new(base_deposit, quote_deposit));
        self
    }

    /// Add a take order on the base side (Immediate-or-Cancel / Market Order)
    pub fn take_base(
        &mut self,
        num_lots: u64,
        min_lots_to_fill: Option<u64>,
        limit: Option<u64>,
    ) -> &mut Self {
        self.base_take = Some(TakeOrder {
            num_lots,
            min_lots_to_fill,
            limit,
        });
        self
    }

    /// Add a take order on the quote side (Immediate-or-Cancel / Market Order)
    pub fn take_quote(
        &mut self,
        num_lots: u64,
        min_lots_to_fill: Option<u64>,
        limit: Option<u64>,
    ) -> &mut Self {
        self.quote_take = Some(TakeOrder {
            num_lots,
            min_lots_to_fill,
            limit,
        });
        self
    }

    /// Add a make order to open a new resting order on a vacant position
    pub fn make_open(&mut self, position: u64, side: LegSide, base_lots: u64) -> &mut Self {
        self.makes
            .push(PositionedMakeOrder::open(position, side, base_lots));
        self
    }

    /// Add a make order to increase lots on an existing occupied resting order
    pub fn make_increase(&mut self, position: u64, base_lots: u64) -> &mut Self {
        self.makes
            .push(PositionedMakeOrder::increase(position, base_lots));
        self
    }

    /// Add a make order to decrease lots on an existing occupied resting order
    pub fn make_decrease(&mut self, position: u64, base_lots: u64) -> &mut Self {
        self.makes
            .push(PositionedMakeOrder::decrease(position, base_lots));
        self
    }

    /// Add a make order to close an existing occupied resting order
    pub fn make_close(&mut self, position: u64, base_lots: u64) -> &mut Self {
        self.makes
            .push(PositionedMakeOrder::close(position, base_lots));
        self
    }

    /// Add an arbitrary positioned make order
    pub fn add_make(&mut self, order: PositionedMakeOrder) -> &mut Self {
        self.makes.push(order);
        self
    }

    /// Build the `MarketCall`
    pub fn build(self) -> Result<MarketCall, GoblinSdkError> {
        let spec = self.spec.ok_or(GoblinSdkError::InvalidTokenPair {
            market: "Unknown",
            base: "Unknown",
            quote: "Unknown",
        })?;
        let locator = self.locator.ok_or(GoblinSdkError::InvalidTokenPair {
            market: "Unknown",
            base: "Unknown",
            quote: "Unknown",
        })?;

        Ok(MarketCall {
            spec,
            locator,
            deposits: self.deposits,
            base_take: self.base_take,
            quote_take: self.quote_take,
            makes: self.makes,
        })
    }
}
