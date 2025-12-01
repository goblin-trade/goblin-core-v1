#![allow(static_mut_refs)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use crate::{
    hostio::HostioContext,
    input_processor::GlobalHeader,
    markets::{DynamicMarket, HardcodedMarket, MarketHeader, ERC20, ETH},
    settlement::global_delta::GlobalDelta,
    types::Pair,
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
pub mod utils;

pub const CONTRACT_ADDRESS: [u8; 20] = [
    0x88, 0x88, 0x41, 0x5d, 0xb8, 0x0e, 0xab, 0xcf, 0x58, 0x02, 0x83, 0xa3, 0xd6, 0x52, 0x49, 0x88,
    0x7d, 0x31, 0x61, 0xb0,
];

static mut HOSTIO_CONTEXT: HostioContext = HostioContext::new();

/// The global delta
///
/// Using `static mut` allows us to take advantage of the fact that lienar memory is zero filled.
/// We get an empty starting buffer without the cost of zeroing.
static mut GLOBAL_DELTA: GlobalDelta = GlobalDelta::new();

fn user_entrypoint_inner(len: usize) -> Result<(), GoblinError> {
    let msg_reentrant = hostio::msg_reentrant();
    require!(!msg_reentrant, GoblinError::Reentrant);

    let ctx = unsafe { &mut HOSTIO_CONTEXT };
    ctx.load();

    let offset = &mut 0usize;
    let global_header = GlobalHeader::new(ctx, offset, len)?;

    // Initialize deltas
    let global_delta = unsafe { &mut GLOBAL_DELTA };
    global_delta
        .global_sender_delta
        .eth_delta
        .set_eth_values(global_header.msg_value, global_header.eth_out_due);

    // Iterate markets
    for _ in 0..global_header.flags.market_count {
        let market_header = MarketHeader::decode(&ctx.args, offset, len)?;

        match market_header.market_type_raw {
            // Hardcoded markets
            HardcodedMarket::<Pair<ETH, ERC20>>::DISCRIMINATOR => {
                HardcodedMarket::<Pair<ETH, ERC20>>::process(
                    ctx,
                    &market_header,
                    global_delta,
                    offset,
                    len,
                )?;
            }

            HardcodedMarket::<Pair<ERC20, ETH>>::DISCRIMINATOR => {
                HardcodedMarket::<Pair<ERC20, ETH>>::process(
                    ctx,
                    &market_header,
                    global_delta,
                    offset,
                    len,
                )?;
            }

            HardcodedMarket::<Pair<ERC20, ERC20>>::DISCRIMINATOR => {
                HardcodedMarket::<Pair<ERC20, ERC20>>::process(
                    ctx,
                    &market_header,
                    global_delta,
                    offset,
                    len,
                )?;
            }

            // Dynamic markets
            DynamicMarket::<Pair<ETH, ERC20>>::DISCRIMINATOR => {
                DynamicMarket::<Pair<ETH, ERC20>>::process(
                    ctx,
                    &market_header,
                    global_delta,
                    global_header.custom_erc20_list,
                    offset,
                    len,
                )?;
            }

            DynamicMarket::<Pair<ERC20, ETH>>::DISCRIMINATOR => {
                DynamicMarket::<Pair<ERC20, ETH>>::process(
                    ctx,
                    &market_header,
                    global_delta,
                    global_header.custom_erc20_list,
                    offset,
                    len,
                )?;
            }

            DynamicMarket::<Pair<ERC20, ERC20>>::DISCRIMINATOR => {
                DynamicMarket::<Pair<ERC20, ERC20>>::process(
                    ctx,
                    &market_header,
                    global_delta,
                    global_header.custom_erc20_list,
                    offset,
                    len,
                )?;
            }
            _ => {}
        }
    }

    // for market_instructions in args.market_instructions_list {
    //     let indexed_market = market_instructions
    //         .market_index
    //         .to_indexed_market(args.custom_market_list)?;

    //     let token_pair =
    //         ValidatedTokenPair::new(indexed_market.token_index_pair, args.custom_erc20_list)?;

    //     // Obtain market key
    //     let market_key = MarketKey::new(
    //         &token_pair,
    //         indexed_market.lot_size_pair,
    //         indexed_market.tick_size,
    //     );

    //     let mut market_state = MarketState::load(&market_key);

    //     // Deltas for sender and makers
    //     let mut sender_delta = SenderDelta::default();
    //     let mut market_maker_deltas = MarketMakerDeltas::default();

    //     if market_instructions.take_bid() {
    //         let match_result = ix_take::<Quote>(
    //             &mut market_maker_deltas,
    //             msg_sender.as_ref(),
    //             &indexed_market,
    //             market_state.as_mut(),
    //             args_buffer.as_ref(),
    //             len,
    //             &mut args.offset,
    //         )?;
    //         sender_delta.quote = match_result;
    //     }

    //     if market_instructions.take_ask() {
    //         let match_result = ix_take::<Base>(
    //             &mut market_maker_deltas,
    //             msg_sender.as_ref(),
    //             &indexed_market,
    //             market_state.as_mut(),
    //             args_buffer.as_ref(),
    //             len,
    //             &mut args.offset,
    //         )?;
    //         sender_delta.base = match_result;
    //     }

    //     // Write market state to slot
    //     market_state.as_mut().store(&market_key);

    //     // Apply pending maker updates
    //     sender_balance_updates.apply_updates(&indexed_market, &sender_delta)?;
    //     maker_balance_updates.apply_updates(&indexed_market, &market_maker_deltas)?;
    // }

    // Settle the deltas
    // sender_balance_updates.settle(
    //     msg_sender.as_ref(),
    //     args.recipient,
    //     args.custom_erc20_list,
    //     args.header.withdraw_internally,
    // )?;

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
