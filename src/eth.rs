use crate::{hostio, quantities::Atoms, types::Address};

/// Transfer out native ETH to a recipient
pub fn transfer_out(recipient: &Address, amount: &Atoms) -> Result<(), ()> {
    let calldata: [u8; 0] = [];
    let return_data_len: &mut usize = &mut 0;

    let call_result = unsafe {
        hostio::call_contract(
            recipient.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            amount.0.as_ptr() as *const u8,
            // Use max gas to follow EVM's CALL 63/64 rule. The VM will decide how much gas to use
            u64::MAX,
            return_data_len,
        )
    };

    if call_result != 0 {
        return Err(());
    }

    Ok(())
}
