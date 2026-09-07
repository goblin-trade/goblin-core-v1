use crate::{
    axis::{Increase, UpdateAddress},
    input_processor::CallerAddresses,
    types::Address,
};

impl UpdateAddress for Increase {
    fn update_address(call_addresses: CallerAddresses) -> &Address {
        // Always debit from caller
        call_addresses.caller
    }
}
