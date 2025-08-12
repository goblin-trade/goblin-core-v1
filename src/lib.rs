#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use crate::{
    input_processor::Args,
    instructions::ix_take,
    settlement::TokenDeltas,
    state::{MarketKey, MarketState, SlotState},
    tokens::ValidatedTokenPair,
    types::Side,
};
use goblin_error::*;

pub mod erc20;
pub mod eth;
pub mod goblin_error;
pub mod hostio;
pub mod input_processor;
pub mod instructions;
pub mod markets;
pub mod matching;
pub mod quantities;
pub mod settlement;
pub mod state;
pub mod tokens;
pub mod types;

pub const CONTRACT_ADDRESS: [u8; 20] = [
    0x88, 0x88, 0x41, 0x5d, 0xb8, 0x0e, 0xab, 0xcf, 0x58, 0x02, 0x83, 0xa3, 0xd6, 0x52, 0x49, 0x88,
    0x7d, 0x31, 0x61, 0xb0,
];

fn user_entrypoint_inner(len: usize) -> Result<(), GoblinError> {
    let msg_reentrant = hostio::msg_reentrant();
    require!(!msg_reentrant, GoblinError::Reentrant);

    let args_buffer = hostio::read_args();
    let mut args = Args::new(args_buffer.as_ref(), len)?;

    let msg_sender = hostio::msg_sender();

    let mut token_deltas = TokenDeltas::new(
        args.header.track_msg_value,
        args.eth_withdrawal_due,
        args.erc20_delta_list,
    )?;

    // TODO execution

    for market_instructions in args.market_instructions_list {
        let indexed_market = market_instructions
            .market_index
            .to_indexed_market(args.custom_market_list)?;

        let token_pair = ValidatedTokenPair::new(
            indexed_market.base_token_index,
            indexed_market.quote_token_index,
            args.custom_erc20_list,
        )?;

        // Obtain market key
        let market_key = MarketKey::new(
            &token_pair,
            indexed_market.base_lot_size,
            indexed_market.quote_lot_size,
            indexed_market.tick_size,
        );

        let mut market_state = MarketState::load(&market_key);

        if market_instructions.take_bid() {
            ix_take(
                market_state.as_mut(),
                Side::Bid,
                args_buffer.as_ref(),
                len,
                &mut args.offset,
            )?;
        }

        // Write market state to slot
        market_state.as_mut().store(&market_key);
    }

    // for _ in 0..args.header.ix_post_only_count {}

    // for _ in 0..args.header.ix_reduce_count {
    //     // Decode bytes one by one
    //     // This instruction has variable number of bytes
    //     ix_reduce_orders(
    //         args_buffer.as_ref(),
    //         len,
    //         &mut args.offset,
    //         args.custom_market_list,
    //         args.custom_erc20_list,
    //         &mut token_deltas,
    //     )?;
    // }

    // for _ in 0..args.header.ix_take_only_count {
    //     ix_take(
    //         args_buffer.as_ref(),
    //         len,
    //         &mut args.offset,
    //         args.custom_market_list,
    //         args.custom_erc20_list,
    //         &mut token_deltas,
    //     )?;
    // }

    // Settlement
    token_deltas.settle(
        msg_sender.as_ref(),
        args.recipient,
        args.custom_erc20_list,
        args.header.withdraw_internally,
        args.header.deposit_shortfall,
    )?;

    // Write cache to trie
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/storage/mod.rs#L81
    hostio::storage_flush_cache(false);

    Ok(())
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    match user_entrypoint_inner(len) {
        Ok(_) => 0,
        Err(err) => err.code(),
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(all(not(test), target_arch = "wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    pay_for_memory_grow(0);
    panic!();
}
