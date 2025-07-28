#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use crate::{input_processor::Args, instructions::ix_reduce_orders, settlement::TokenDeltas};
use goblin_error::*;
use hostio::*;

pub mod erc20;
pub mod eth;
pub mod goblin_error;
pub mod hostio;
pub mod input_processor;
pub mod instructions;
pub mod market_params;
pub mod markets;
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
    let msg_reentrant = unsafe { hostio::msg_reentrant() };
    require!(!msg_reentrant, GoblinError::Reentrant);

    let args_buffer = unsafe { hostio_helpers::hostio_read_args() };

    // Ideally args should remain immutable. We only need a variable offset.
    // This is going to cause some problem, TODO update.
    let mut args = Args::new(args_buffer.as_ref(), len)?;

    let msg_sender = unsafe { hostio_msg_sender() };

    let mut token_deltas = TokenDeltas::new(
        args.header.track_msg_value,
        args.eth_withdrawal_due,
        args.erc20_delta_list,
    )?;

    // TODO execution
    for _ in 0..args.header.ix_post_only_count {}

    for _ in 0..args.header.ix_reduce_count {
        // Decode bytes one by one
        // This instruction has variable number of bytes
        ix_reduce_orders(
            args_buffer.as_ref(),
            len,
            &mut args.offset,
            args.custom_market_list,
            args.custom_erc20_list,
            &mut token_deltas,
        )?;
    }

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
    unsafe {
        hostio::storage_flush_cache(false);
    }

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
