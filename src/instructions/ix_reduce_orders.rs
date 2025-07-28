use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::IndexedMarket,
    quantities::{BaseLots, Delta, Ticks},
    require,
    settlement::TokenDeltas,
    state::{MarketKey, MarketState, SlotState},
    tokens::ValidatedTokenPair,
    types::Address,
};

struct ReduceOrdersHeader {
    pub market_index: u8,

    pub bid_outer_indices: u8,

    pub ask_outer_indices: u8,
}

impl ReduceOrdersHeader {
    const BYTE_SIZE: usize = 2;

    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        require!(
            len >= *offset + Self::BYTE_SIZE,
            GoblinError::InvalidPayload
        );
        let header = Self {
            market_index: payload[*offset],
            bid_outer_indices: payload[*offset + 1] & 0b0000_1111,
            ask_outer_indices: (payload[*offset + 1] & 0b1111_0000) >> 4,
        };
        *offset += Self::BYTE_SIZE;

        Ok(header)
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MatrixHeader {
    pub outer_index: u16,
    pub order_count: u8,
}

impl MatrixHeader {
    const BYTE_SIZE: usize = core::mem::size_of::<Self>();

    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        require!(
            len >= *offset + Self::BYTE_SIZE,
            GoblinError::InvalidPayload
        );
        let header = payload.decode_ref::<MatrixHeader>(*offset);
        *offset += Self::BYTE_SIZE;

        Ok(*header)
    }
}

#[repr(C, packed)]
pub struct ReduceOrderPacket {
    pub row_index: u8,    // 5 bits
    pub column_index: u8, // 3 bits
    pub size: BaseLots,
}

impl ReduceOrderPacket {
    const BYTE_SIZE: usize = 1 + core::mem::size_of::<BaseLots>();

    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        require!(
            len >= *offset + Self::BYTE_SIZE,
            GoblinError::InvalidPayload
        );
        let packet = Self {
            row_index: (payload[*offset] & 0b1111_1000) >> 5,
            column_index: payload[*offset] & 0b0000_0111,
            size: BaseLots(u64::from_le_bytes([
                payload[*offset + 1],
                payload[*offset + 2],
                payload[*offset + 3],
                payload[*offset + 4],
                payload[*offset + 5],
                payload[*offset + 6],
                payload[*offset + 7],
                payload[*offset + 8],
            ])),
        };
        *offset += Self::BYTE_SIZE;

        Ok(packet)
    }
}

/// Reduce resting orders in a market. Set amount to MAX will cancel the order.
pub fn ix_reduce_orders(
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
    custom_market_list: &[IndexedMarket],
    custom_erc20_list: &[Address],
    token_deltas: &mut TokenDeltas,
) -> Result<(), GoblinError> {
    let header = ReduceOrdersHeader::decode(payload, len, offset)?;

    let indexed_market =
        IndexedMarket::from_index(header.market_index as usize, custom_market_list)?;

    let token_pair = ValidatedTokenPair::new(
        indexed_market.base_token_index,
        indexed_market.quote_token_index,
        custom_erc20_list,
    )?;

    // Obtain market key
    let market_key = MarketKey::new(
        &token_pair,
        indexed_market.base_lot_size,
        indexed_market.quote_lot_size,
        indexed_market.tick_size,
    );

    // Read and store market state
    let mut market_state = MarketState::load(&market_key);
    market_state.as_mut().store(&market_key);

    for _ in 0..header.bid_outer_indices {
        let matrix_header = MatrixHeader::decode(payload, len, offset)?;

        for _ in 0..matrix_header.order_count {
            let ReduceOrderPacket {
                row_index: inner_index,
                column_index: row_index,
                size,
            } = ReduceOrderPacket::decode(payload, len, offset)?;

            // Feed to remover
        }
    }

    // Mock amounts that need to be settled
    let base_delta = Delta(10);
    let quote_delta = Delta(-4);

    // TokenIndex to Token address lookup happens again in settlement phase.
    token_deltas.add_consumed_amount(indexed_market.base_token_index, base_delta)?;
    token_deltas.add_consumed_amount(indexed_market.quote_token_index, quote_delta)?;

    Ok(())
}
