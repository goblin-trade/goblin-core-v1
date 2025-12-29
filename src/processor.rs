use crate::{
    goblin_error::GoblinError,
    hostio::{self, HostioContext},
    input_processor::{Decodable, DecodeCtx, GlobalHeader},
    require,
    settlement::Delta,
};

pub const CONTRACT_ADDRESS: [u8; 20] = [
    0x88, 0x88, 0x41, 0x5d, 0xb8, 0x0e, 0xab, 0xcf, 0x58, 0x02, 0x83, 0xa3, 0xd6, 0x52, 0x49, 0x88,
    0x7d, 0x31, 0x61, 0xb0,
];

/// The call args and msg_sender. Initially zero filled.
///
/// `static mut` allows us to take advantage of the fact that lienar memory is zero filled.
/// We get an empty starting buffer without the cost of zeroing.
static mut HOSTIO_CONTEXT: HostioContext = HostioContext::zero();

/// Delta, initially zero filled.
static mut DELTA: Delta = Delta::zero();

pub fn processor(len: usize) -> Result<(), GoblinError> {
    let msg_reentrant = hostio::msg_reentrant();
    require!(!msg_reentrant, GoblinError::Reentrant);

    let hostio_ctx = unsafe { &mut HOSTIO_CONTEXT };
    hostio_ctx.load();

    let ctx = &mut DecodeCtx::new(&hostio_ctx.args, len);
    let global_header = GlobalHeader::try_decode(ctx)?;
    let delta = unsafe { &mut DELTA };

    global_header.market_counts.process_markets(
        ctx,
        &hostio_ctx.msg_sender,
        global_header.custom_erc20_list,
        delta,
    )?;

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
