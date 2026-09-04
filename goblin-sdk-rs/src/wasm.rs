#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use goblin_core_v1::{
    axis::leg::LegEnum,
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, Column, InnerPos, OuterBitmapIndex, OuterPos, Position,
        QuoteLots, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, Ticks, UnsidedAtoms,
        UnsidedDeltaLots,
    },
};

#[cfg(feature = "wasm")]
use crate::{
    builder::{GoblinCalldataBuilder, MarketCallBuilder},
    position::{
        column_from_position, position_from_parts, position_from_ticks, ticks_from_position,
    },
    types::{MarketSpecIndex, TokenKind},
};

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmMarketCallBuilder {
    pub(crate) inner: MarketCallBuilder,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmMarketCallBuilder {
    #[wasm_bindgen(js_name = newHardcoded)]
    pub fn new_hardcoded(
        spec_index: u8,
        market_index: u8,
    ) -> Result<WasmMarketCallBuilder, JsValue> {
        let spec = match spec_index {
            0 => MarketSpecIndex::HardcodedEthHardcodedERC20,
            1 => MarketSpecIndex::HardcodedHardcodedERC20Eth,
            2 => MarketSpecIndex::HardcodedHardcodedERC20HardcodedERC20,
            _ => {
                return Err(JsValue::from_str(
                    "Invalid hardcoded market spec index (must be 0..2)",
                ))
            }
        };

        let mut inner = MarketCallBuilder::new();
        inner
            .hardcoded(spec, market_index)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = newDynamic)]
    pub fn new_dynamic(
        base_kind: u8,
        base_token_index: u8,
        quote_kind: u8,
        quote_token_index: u8,
        base_lot_size: u64,
        quote_lot_size: u64,
        tick_size: u64,
    ) -> Result<WasmMarketCallBuilder, JsValue> {
        let base = match base_kind {
            0 => TokenKind::ETH,
            1 => TokenKind::HardcodedERC20,
            2 => TokenKind::CustomERC20,
            _ => {
                return Err(JsValue::from_str(
                    "Invalid base token kind (0=ETH, 1=HardcodedERC20, 2=CustomERC20)",
                ))
            }
        };
        let quote = match quote_kind {
            0 => TokenKind::ETH,
            1 => TokenKind::HardcodedERC20,
            2 => TokenKind::CustomERC20,
            _ => {
                return Err(JsValue::from_str(
                    "Invalid quote token kind (0=ETH, 1=HardcodedERC20, 2=CustomERC20)",
                ))
            }
        };

        let mut inner = MarketCallBuilder::new();
        inner
            .dynamic(
                base,
                base_token_index,
                quote,
                quote_token_index,
                BaseLotsPerBaseUnit::new(base_lot_size),
                QuoteLotsPerQuoteUnit::new(quote_lot_size),
                QuoteLotsPerBaseUnitPerTick::new(tick_size),
            )
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = deposit)]
    pub fn deposit(&mut self, base_deposit: i64, quote_deposit: i64) {
        self.inner.deposit(
            UnsidedDeltaLots::new(base_deposit),
            UnsidedDeltaLots::new(quote_deposit),
        );
    }

    #[wasm_bindgen(js_name = takeBase)]
    pub fn take_base(&mut self, num_lots: u64, min_lots: Option<u64>, limit: Option<u64>) {
        self.inner.take_base(
            BaseLots::new(num_lots),
            min_lots.map(BaseLots::new),
            limit.map(Position::new),
        );
    }

    #[wasm_bindgen(js_name = takeQuote)]
    pub fn take_quote(&mut self, num_lots: u64, min_lots: Option<u64>, limit: Option<u64>) {
        self.inner.take_quote(
            QuoteLots::new(num_lots),
            min_lots.map(QuoteLots::new),
            limit.map(Position::new),
        );
    }

    #[wasm_bindgen(js_name = makeOpen)]
    pub fn make_open(&mut self, position: u64, side: u8, base_lots: u64) -> Result<(), JsValue> {
        let leg_side = match side {
            0 => LegEnum::Base,
            1 => LegEnum::Quote,
            _ => return Err(JsValue::from_str("Invalid leg side (0=Base, 1=Quote)")),
        };
        self.inner
            .make_open(Position::new(position), leg_side, BaseLots::new(base_lots));
        Ok(())
    }

    #[wasm_bindgen(js_name = makeIncrease)]
    pub fn make_increase(&mut self, position: u64, base_lots: u64) {
        self.inner
            .make_increase(Position::new(position), BaseLots::new(base_lots));
    }

    #[wasm_bindgen(js_name = makeDecrease)]
    pub fn make_decrease(&mut self, position: u64, base_lots: u64) {
        self.inner
            .make_decrease(Position::new(position), BaseLots::new(base_lots));
    }

    #[wasm_bindgen(js_name = makeClose)]
    pub fn make_close(&mut self, position: u64, base_lots: u64) {
        self.inner
            .make_close(Position::new(position), BaseLots::new(base_lots));
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmGoblinCalldataBuilder {
    inner: GoblinCalldataBuilder,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmGoblinCalldataBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: GoblinCalldataBuilder::new(),
        }
    }

    #[wasm_bindgen(js_name = setRecipient)]
    pub fn set_recipient(&mut self, recipient_hex: &str) -> Result<(), JsValue> {
        self.inner
            .set_recipient_hex(recipient_hex)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(())
    }

    #[wasm_bindgen(js_name = setMsgValue)]
    pub fn set_msg_value(&mut self, read_msg_value: bool) {
        self.inner.set_msg_value(read_msg_value);
    }

    #[wasm_bindgen(js_name = setEthWithdrawal)]
    pub fn set_eth_withdrawal(&mut self, amount: u64, internally: bool) {
        self.inner
            .set_eth_withdrawal(UnsidedAtoms::new(amount), internally);
    }

    #[wasm_bindgen(js_name = addCustomToken)]
    pub fn add_custom_token(&mut self, address_hex: &str) -> Result<u8, JsValue> {
        self.inner
            .add_custom_token_hex(address_hex)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = addMarketCall)]
    pub fn add_market_call(
        &mut self,
        market_builder: WasmMarketCallBuilder,
    ) -> Result<(), JsValue> {
        let call = market_builder
            .inner
            .build()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.inner.add_market(call);
        Ok(())
    }

    #[wasm_bindgen(js_name = build)]
    pub fn build(&self) -> Result<Vec<u8>, JsValue> {
        self.inner
            .build()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = buildHex)]
    pub fn build_hex(&self) -> Result<String, JsValue> {
        self.inner
            .build_hex()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = wasmPositionFromParts)]
pub fn wasm_position_from_parts(outer_bitmap_index: u64, outer_pos: u8, inner_pos: u8) -> u64 {
    position_from_parts(
        OuterBitmapIndex::new(outer_bitmap_index),
        OuterPos::new(outer_pos),
        InnerPos::new(inner_pos),
    )
    .inner
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = wasmPositionFromTicks)]
pub fn wasm_position_from_ticks(ticks: u64, column: u8) -> u64 {
    position_from_ticks(Ticks::new(ticks), Column::new(column)).inner
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = wasmTicksFromPosition)]
pub fn wasm_ticks_from_position(position: u64) -> u64 {
    ticks_from_position(Position::new(position)).inner
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = wasmColumnFromPosition)]
pub fn wasm_column_from_position(position: u64) -> u8 {
    column_from_position(Position::new(position)).inner
}
