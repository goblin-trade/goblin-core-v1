use deku::DekuRead;
use goblin_macros::ConstDefault;

mod impl_checked_ops;
mod impl_deku_reader;
#[cfg(feature = "encode")]
mod impl_deku_writer;
mod impl_from;
mod impl_index;
mod impl_into_iterator;

/// Stub type for ETH token index, address and deposit
///
/// Use an explicit stub type instead of `()` for clarity
#[derive(Default, Clone, Copy, PartialEq, ConstDefault, DekuRead)]
#[cfg_attr(feature = "encode", derive(deku::DekuWrite))]
pub struct ETHStub;
