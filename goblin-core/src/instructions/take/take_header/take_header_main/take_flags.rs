use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;
use deku::ctx::Order;

/// Two flags packed into the take header's flag field, LSB first.
///
/// The bit order is taken from `ctx` so this stays in lockstep with the
/// enclosing [`TakeHeaderMain`](super::TakeHeaderMain) bit order.
#[derive(Clone, Copy, DekuRead)]
#[deku(bit_order = "order", ctx = "order: Order")]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct TakeFlags {
    #[deku(bits = "1")]
    pub read_min_lots: bool,
    #[deku(bits = "1")]
    pub read_limit: bool,
}
