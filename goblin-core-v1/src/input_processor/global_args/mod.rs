pub mod caller_addresses;
pub mod global_args;
pub mod global_header;
pub mod msg_transfers;

pub use caller_addresses::*;
pub use global_args::*;
pub use global_header::*;
pub use msg_transfers::*;

mod hostio_fields;
mod impl_compound_decode;
