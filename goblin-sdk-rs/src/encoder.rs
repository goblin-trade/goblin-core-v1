use goblin_core_v1::{
    quantities::{InnerPos, Position, UnsidedAtoms},
    types::Address,
};

use crate::{
    error::GoblinSdkError,
    position::group_makes_by_bitmaps,
    types::{BaseTakeOrder, MakeAction, MarketCall, MarketLocator, QuoteTakeOrder, TokenKind},
};

/// Configuration for the global calldata payload
#[derive(Debug, Clone, Default)]
pub struct GlobalPayloadConfig {
    /// Optional custom recipient address
    pub custom_recipient: Option<Address>,
    /// Whether hostio should read msg.value
    pub read_msg_value: bool,
    /// ETH withdrawal amount in atoms
    pub withdraw_eth_amount: UnsidedAtoms,
    /// Whether to credit withdrawal internally (store credit) instead of external transfer
    pub withdraw_internally: bool,
    /// List of custom ERC20 token addresses (up to 7)
    pub custom_tokens: Vec<Address>,
    /// Markets to execute, grouped by their respective MarketSpecIndex
    pub markets: Vec<MarketCall>,
}

/// Low-level encoder for goblin-core-v1 calldata
pub struct GoblinEncoder;

impl GoblinEncoder {
    /// Serialize a complete global payload with all markets and actions
    pub fn encode_payload(config: &GlobalPayloadConfig) -> Result<Vec<u8>, GoblinSdkError> {
        if config.custom_tokens.len() > 7 {
            return Err(GoblinSdkError::CustomTokenLimitExceeded(
                config.custom_tokens.len(),
            ));
        }

        // Group markets by MarketSpecIndex
        let mut markets_by_spec: [Vec<&MarketCall>; 11] = Default::default();
        let mut has_dynamic_markets = false;

        for market in &config.markets {
            let spec_idx = market.spec.index();
            markets_by_spec[spec_idx].push(market);
            if market.spec.is_dynamic() {
                has_dynamic_markets = true;
            }
        }

        // Validate market counts (max 15 per spec, fits in 4 bits)
        let mut counts = [0u8; 11];
        for i in 0..11 {
            let len = markets_by_spec[i].len();
            if len > 15 {
                return Err(GoblinSdkError::MarketCountLimitExceeded(i, len));
            }
            counts[i] = len as u8;
        }

        let withdraw_eth = config.withdraw_eth_amount.inner > 0;
        let read_custom_recipient = config.custom_recipient.is_some();
        let custom_erc20_count = config.custom_tokens.len();

        let mut out = Vec::new();

        // 1. Header Flags (1 byte)
        let byte_0: u8 = (read_custom_recipient as u8)
            | ((config.read_msg_value as u8) << 1)
            | ((has_dynamic_markets as u8) << 2)
            | ((withdraw_eth as u8) << 3)
            | ((config.withdraw_internally as u8) << 4)
            | (((custom_erc20_count as u8) & 0x07) << 5);
        out.push(byte_0);

        // 2. Global Header
        // 2a. ETH withdrawal amount if enabled (8 bytes LE)
        if withdraw_eth {
            out.extend_from_slice(&config.withdraw_eth_amount.inner.to_le_bytes());
        }

        // 2b. Custom recipient address if enabled (20 bytes)
        if let Some(recipient) = config.custom_recipient {
            out.extend_from_slice(&recipient);
        }

        // 2c. Market counts
        // 2 bytes for hardcoded counts (specs 0, 1, 2)
        let hc_byte_0 = (counts[0] & 0x0F) | ((counts[1] & 0x0F) << 4);
        let hc_byte_1 = counts[2] & 0x0F;
        out.push(hc_byte_0);
        out.push(hc_byte_1);

        // If dynamic markets enabled, 4 bytes for specs 3..11
        if has_dynamic_markets {
            let dyn_byte_2 = (counts[3] & 0x0F) | ((counts[4] & 0x0F) << 4);
            let dyn_byte_3 = (counts[5] & 0x0F) | ((counts[6] & 0x0F) << 4);
            let dyn_byte_4 = (counts[7] & 0x0F) | ((counts[8] & 0x0F) << 4);
            let dyn_byte_5 = (counts[9] & 0x0F) | ((counts[10] & 0x0F) << 4);
            out.push(dyn_byte_2);
            out.push(dyn_byte_3);
            out.push(dyn_byte_4);
            out.push(dyn_byte_5);
        }

        // 2d. Custom ERC20 tokens list (20 bytes each)
        for token_address in &config.custom_tokens {
            out.extend_from_slice(token_address);
        }

        // 3. Markets in order of the 11 specs
        for spec_idx in 0..11 {
            for market in &markets_by_spec[spec_idx] {
                Self::encode_market_call(market, config.custom_tokens.len(), &mut out)?;
            }
        }

        Ok(out)
    }

    /// Encode an individual market call
    pub fn encode_market_call(
        market: &MarketCall,
        custom_tokens_count: usize,
        out: &mut Vec<u8>,
    ) -> Result<(), GoblinSdkError> {
        let spec = market.spec;
        let base_kind = spec.base_token();
        let quote_kind = spec.quote_token();

        // Bitmap grouping for makes
        let outer_groups = group_makes_by_bitmaps(&market.makes)?;
        let outer_bitmap_count = outer_groups.len() as u8;
        if outer_bitmap_count > 3 {
            return Err(GoblinSdkError::OuterBitmapLimitExceeded(
                outer_bitmap_count as usize,
            ));
        }

        let decode_deposit_amounts = match market.deposits {
            Some(d) => !d.is_zero(),
            None => false,
        };

        let has_base_take = market.base_take.is_some();
        let has_quote_take = market.quote_take.is_some();

        // 1. Market Header (1 byte)
        let header_byte: u8 = (decode_deposit_amounts as u8)
            | ((has_base_take as u8) << 1)
            | ((has_quote_take as u8) << 2)
            | ((outer_bitmap_count & 0x03) << 3);
        out.push(header_byte);

        // 2. Local Deposits (if enabled)
        if decode_deposit_amounts {
            let deposits = market.deposits.unwrap_or_default();
            // Base deposit (only if ERC20)
            if base_kind.is_erc20() {
                out.extend_from_slice(&deposits.base_deposit.inner.to_le_bytes());
            }
            // Quote deposit (only if ERC20)
            if quote_kind.is_erc20() {
                out.extend_from_slice(&deposits.quote_deposit.inner.to_le_bytes());
            }
        }

        // 3. Market Locator
        match market.locator {
            MarketLocator::Hardcoded { market_index } => {
                out.push(market_index);
            }
            MarketLocator::Dynamic {
                base_token_index,
                quote_token_index,
                base_lot_size,
                quote_lot_size,
                tick_size,
            } => {
                // Token index pair
                if base_kind.is_erc20() {
                    if base_kind == TokenKind::CustomERC20
                        && base_token_index as usize >= custom_tokens_count
                    {
                        return Err(GoblinSdkError::CustomTokenIndexOutOfRange(
                            base_token_index,
                            custom_tokens_count,
                        ));
                    }
                    out.push(base_token_index);
                }
                if quote_kind.is_erc20() {
                    if quote_kind == TokenKind::CustomERC20
                        && quote_token_index as usize >= custom_tokens_count
                    {
                        return Err(GoblinSdkError::CustomTokenIndexOutOfRange(
                            quote_token_index,
                            custom_tokens_count,
                        ));
                    }
                    out.push(quote_token_index);
                }

                // Lot sizes and tick size (8 bytes each LE)
                out.extend_from_slice(&base_lot_size.inner.to_le_bytes());
                out.extend_from_slice(&quote_lot_size.inner.to_le_bytes());
                out.extend_from_slice(&tick_size.inner.to_le_bytes());
            }
        }

        // 4. Takes (Base first, then Quote)
        if let Some(ref base_take) = market.base_take {
            Self::encode_take_base(base_take, out)?;
        }
        if let Some(ref quote_take) = market.quote_take {
            Self::encode_take_quote(quote_take, out)?;
        }

        // 5. Makes (Hierarchical outer bitmaps)
        for outer_group in &outer_groups {
            out.extend_from_slice(&outer_group.outer_bitmap_index.inner.to_le_bytes());
            out.push(outer_group.inner_bitmaps.len() as u8);

            for inner_group in &outer_group.inner_bitmaps {
                out.push(inner_group.outer_pos.inner);
                out.push(inner_group.updates.len() as u8);

                for (inner_pos, action) in &inner_group.updates {
                    Self::encode_make(*inner_pos, action, out)?;
                }
            }
        }

        Ok(())
    }

    /// Encode a base take order
    pub fn encode_take_base(take: &BaseTakeOrder, out: &mut Vec<u8>) -> Result<(), GoblinSdkError> {
        Self::encode_take_raw(
            take.num_lots.inner,
            take.min_lots_to_fill.map(|l| l.inner),
            take.limit,
            out,
        )
    }

    /// Encode a quote take order
    pub fn encode_take_quote(
        take: &QuoteTakeOrder,
        out: &mut Vec<u8>,
    ) -> Result<(), GoblinSdkError> {
        Self::encode_take_raw(
            take.num_lots.inner,
            take.min_lots_to_fill.map(|l| l.inner),
            take.limit,
            out,
        )
    }

    fn encode_take_raw(
        num_lots: u64,
        min_lots_to_fill: Option<u64>,
        limit: Option<Position>,
        out: &mut Vec<u8>,
    ) -> Result<(), GoblinSdkError> {
        if num_lots == 0 {
            return Err(GoblinSdkError::ZeroTakeLots);
        }
        if num_lots >= (1u64 << 62) {
            return Err(GoblinSdkError::TakeLotsOverflow(num_lots));
        }

        let read_min_lots = min_lots_to_fill.is_some();
        let read_limit = limit.is_some();

        if let Some(lim) = limit {
            if lim == Position::ZERO {
                return Err(GoblinSdkError::ZeroLimit);
            }
        }

        let raw_bytes = (num_lots << 2) | ((read_limit as u64) << 1) | (read_min_lots as u64);
        out.extend_from_slice(&raw_bytes.to_le_bytes());

        if let Some(min_lots) = min_lots_to_fill {
            out.extend_from_slice(&min_lots.to_le_bytes());
        }

        if let Some(lim) = limit {
            out.extend_from_slice(&lim.inner.to_le_bytes());
        }

        Ok(())
    }

    /// Encode a single make order update
    pub fn encode_make(
        inner_pos: InnerPos,
        action: &MakeAction,
        out: &mut Vec<u8>,
    ) -> Result<(), GoblinSdkError> {
        let base_lots_raw = action.base_lots().inner;
        if base_lots_raw >= (1u64 << 62) {
            return Err(GoblinSdkError::MakeLotsOverflow(base_lots_raw));
        }

        out.push(inner_pos.inner);

        let (occupancy_bit, inner_enum_raw_bit) = action.encode_bits();
        let raw_bytes =
            (base_lots_raw << 2) | ((inner_enum_raw_bit as u64) << 1) | (occupancy_bit as u64);

        out.extend_from_slice(&raw_bytes.to_le_bytes());
        Ok(())
    }
}
