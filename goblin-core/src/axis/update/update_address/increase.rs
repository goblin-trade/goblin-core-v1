use crate::{
    axis::update::{Increase, UpdateAddress},
    input_processor::CallerAddresses,
    types::Address,
};

impl UpdateAddress for Increase {
    fn get_update_address<'a>(call_addresses: CallerAddresses<'a>) -> &'a Address {
        // Always debit from caller
        call_addresses.caller
    }
}
