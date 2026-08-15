use crate::axis::update::{update_make::UpdateMake, Decrease, UpdateEnum};

impl UpdateMake for Decrease {
    const UPDATE_ENUM: UpdateEnum = UpdateEnum::Decrease;
}
