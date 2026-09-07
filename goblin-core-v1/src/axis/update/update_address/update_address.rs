use crate::{input_processor::CallerAddresses, types::Address};

pub trait UpdateAddress {
    fn update_address(call_addresses: CallerAddresses) -> &Address;
}
