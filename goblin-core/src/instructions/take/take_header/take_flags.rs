/// Two flags packed into the take header's flag field, LSB first.
///
/// The field is part of [`TakeHeader`](super::TakeHeader)'s bit lane, so it has
/// no standalone codec impl: it is sliced out of the lane there.
#[derive(Clone, Copy)]
pub struct TakeFlags {
    pub read_min_lots: bool,
    pub read_limit: bool,
}
