use crate::{axis::CallerMarker, types::Address};

#[derive(Clone, Copy)]
pub struct CallerData<'a, CM: CallerMarker> {
    pub address: &'a Address,
    pub locator: CM::Locator,
}
