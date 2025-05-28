#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem::MaybeUninit;
use getter::*;
use goblin_error::*;
use hostio::*;
use input_processor::{read_token_deltas, CallHeader, CallPayload};
use instructions::*;
use quantities::Delta;
use settlement::{EthDelta, IndexedTokenDelta, TokenDeltaList};
use types::Address;

pub mod erc20;
pub mod eth;
pub mod events;
pub mod getter;
pub mod goblin_error;
pub mod hostio;
pub mod input_processor;
pub mod instructions;
pub mod market_params;
pub mod quantities;
pub mod settlement;
pub mod state;
pub mod tokens;
pub mod types;

#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub mod indexer_hostio;

pub const ADDRESS: [u8; 20] = [
    0x88, 0x88, 0x41, 0x5d, 0xb8, 0x0e, 0xab, 0xcf, 0x58, 0x02, 0x83, 0xa3, 0xd6, 0x52, 0x49, 0x88,
    0x7d, 0x31, 0x61, 0xb0,
];

fn user_entrypoint_inner(len: usize) -> Result<(), GoblinError> {
    let msg_reentrant = unsafe { hostio::msg_reentrant() };
    require!(!msg_reentrant, GoblinError::Reentrant);

    let payload = &mut CallPayload::new(len);
    let header = CallHeader::init(payload)?;

    let mut msg_sender_maybe = MaybeUninit::<Address>::uninit();
    let msg_sender = unsafe {
        hostio::msg_sender(msg_sender_maybe.as_mut_ptr() as *mut u8);
        msg_sender_maybe.assume_init_ref()
    };

    let recipient =
        input_processor::read_recipient(header.recipient_provided, payload, msg_sender)?;

    let mut eth_delta = EthDelta::init(header.track_eth_delta, payload)?;

    let custom_tokens = input_processor::read_custom_tokens(header.custom_token_count, payload)?;
    let token_delta_list = input_processor::read_token_deltas(header.token_delta_count, payload)?;

    // eth_delta.settle(&msg_sender, recipient, header.withdraw_internally)?;

    // TODO settle token_delta_list

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
