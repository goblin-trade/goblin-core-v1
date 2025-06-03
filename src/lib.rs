#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem::MaybeUninit;
use getter::*;
use goblin_error::*;
use hostio::*;
use hostio_buffer::HostioBuffer;
use input_processor::{read_token_deltas, CallHeader, CallPayload};
use instructions::*;
use quantities::Delta;
use settlement::{EthDelta, TokenWithdrawalDue, TokensConsumedList};
use types::Address;

use crate::{input_processor::DecodedPayload, settlement::eth_delta};

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

pub const ADDRESS: [u8; 20] = [
    0x88, 0x88, 0x41, 0x5d, 0xb8, 0x0e, 0xab, 0xcf, 0x58, 0x02, 0x83, 0xa3, 0xd6, 0x52, 0x49, 0x88,
    0x7d, 0x31, 0x61, 0xb0,
];

fn user_entrypoint_inner(len: usize) -> Result<(), GoblinError> {
    let msg_reentrant = unsafe { hostio::msg_reentrant() };
    require!(!msg_reentrant, GoblinError::Reentrant);

    let payload = CallPayload::new(len);
    let decoded_payload = DecodedPayload::new(&payload)?;

    let msg_sender = unsafe { hostio_msg_sender() };

    let eth_delta = EthDelta::init(
        decoded_payload.header.track_msg_value,
        decoded_payload.eth_withdrawal_due,
    )?;

    let recipient = match decoded_payload.provided_recipient {
        Some(provided_recipient) => provided_recipient,
        None => msg_sender.as_ref(),
    };

    eth_delta.settle(
        msg_sender.as_ref(),
        recipient,
        decoded_payload.header.withdraw_internally,
    )?;

    // token_delta_list
    //     .get(0)
    //     .unwrap()
    //     .settle(custom_token_list, msg_sender.as_ref(), &recipient)?;

    // for delta in token_delta_list {
    //     delta.settle(custom_token_list, msg_sender.as_ref(), &recipient)?;
    // }

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
