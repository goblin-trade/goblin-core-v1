use crate::axis::update::{update_make::UpdateMake, Increase, UpdateEnum};

impl UpdateMake for Increase {
    const UPDATE_ENUM: UpdateEnum = UpdateEnum::Increase;
}
