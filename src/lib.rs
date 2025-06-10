#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use crate::{input_processor::DecodedPayload, settlement::ERC20DeltaList};
use goblin_error::*;
use hostio::*;
use input_processor::CallPayload;
use settlement::EthDelta;

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

pub const CONTRACT_ADDRESS: [u8; 20] = [
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

    let mut erc20_delta_list = ERC20DeltaList::init(
        decoded_payload.erc20_withdrawals_bytes,
        decoded_payload.custom_token_list,
    )?;

    // TODO execution

    // Settlement

    eth_delta.settle(
        msg_sender.as_ref(),
        decoded_payload.recipient,
        decoded_payload.header.withdraw_internally,
    )?;

    for erc20_delta in erc20_delta_list.iter_mut() {
        erc20_delta.settle(
            msg_sender.as_ref(),
            decoded_payload.recipient,
            decoded_payload.header.deposit_shortfall,
            decoded_payload.header.withdraw_internally,
        )?;
    }

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
