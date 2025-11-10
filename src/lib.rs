#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use crate::{
    input_processor::{Args, ArgsDecoder, Decodable},
    instructions::ix_take,
    markets::{DynamicMarket, GoblinMarket, HardcodedMarket, MarketHeader},
    settlement::{MakerBalanceUpdates, MarketMakerDeltas, SenderBalanceUpdates, SenderDelta},
    state::{MarketState, SlotState},
    tokens::{
        DynamicIndex, HardcodedIndex, HardcodedMarketList, HardcodedToken, MarketVariant,
        TokenIndex, TokenPair, TokenPairDecoder, ERC20, ETH,
    },
    types::{Base, Pair, Quote},
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

fn user_entrypoint_inner(len: usize) -> Result<(), GoblinError> {
    // Re-entrancy disabled
    let msg_reentrant = hostio::msg_reentrant();
    require!(!msg_reentrant, GoblinError::Reentrant);

    // Read args and sender
    let args_buffer = hostio::read_args();
    let mut args = Args::new(args_buffer.as_ref(), len)?;
    let msg_sender = hostio::msg_sender();

    // Initialize deltas
    let mut sender_balance_updates = SenderBalanceUpdates::new(args.msg_value, args.eth_out_due);
    let mut maker_balance_updates = MakerBalanceUpdates::default();

    // Iterate markets
    for _ in 0..args.header.market_count {
        let market_header = MarketHeader::decode(args_buffer.as_ref(), &mut args.offset, len)?;

        // Currently there is no blanket impl for HardcodedMarket and DynamicMarket because
        // only DynamicMarket uses custom_erc20_list.
        // If we need to add this field on HardcodedMarket, we can use a blanket impl.
        //
        // TODO add withdraw and deposit logic that is namespaced by market
        // - ETH: withdraw only. 64 bit.
        // - ERC20: withdraw or deposit. 64 bit.
        //
        // Header flag will tell us whether to read deposit / withdraw amounts.
        //
        // No need to have if-else to separate hardcoded and custom token deposits. They are already
        // namespaced by market type.
        // But dynamic markets have dynamic token index- could be hardcoded or custom.
        match market_header.market_type_raw {
            // Hardcoded markets
            HardcodedMarket::<Pair<ETH, ERC20>>::DISCRIMINATOR => {
                HardcodedMarket::<Pair<ETH, ERC20>>::process(
                    args_buffer.as_ref(),
                    &mut args.offset,
                    len,
                )?;
            }

            HardcodedMarket::<Pair<ERC20, ETH>>::DISCRIMINATOR => {
                HardcodedMarket::<Pair<ERC20, ETH>>::process(
                    args_buffer.as_ref(),
                    &mut args.offset,
                    len,
                )?;
            }

            HardcodedMarket::<Pair<ERC20, ERC20>>::DISCRIMINATOR => {
                HardcodedMarket::<Pair<ERC20, ERC20>>::process(
                    args_buffer.as_ref(),
                    &mut args.offset,
                    len,
                )?;
            }

            // Dynamic markets
            DynamicMarket::<Pair<ETH, ERC20>>::DISCRIMINATOR => {
                DynamicMarket::<Pair<ETH, ERC20>>::process(
                    args_buffer.as_ref(),
                    &mut args.offset,
                    len,
                    args.custom_erc20_list,
                )?;
            }

            DynamicMarket::<Pair<ERC20, ETH>>::DISCRIMINATOR => {
                DynamicMarket::<Pair<ERC20, ETH>>::process(
                    args_buffer.as_ref(),
                    &mut args.offset,
                    len,
                    args.custom_erc20_list,
                )?;
            }

            DynamicMarket::<Pair<ERC20, ERC20>>::DISCRIMINATOR => {
                DynamicMarket::<Pair<ERC20, ERC20>>::process(
                    args_buffer.as_ref(),
                    &mut args.offset,
                    len,
                    args.custom_erc20_list,
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
    sender_balance_updates.settle(
        msg_sender.as_ref(),
        args.recipient,
        args.custom_erc20_list,
        args.header.withdraw_internally,
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
