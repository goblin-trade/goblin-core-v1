use crate::axis::update::{update_make::UpdateMake, Decrease, Increase};

pub trait UpdateMarker: UpdateMake {}

impl UpdateMarker for Increase {}
impl UpdateMarker for Decrease {}
