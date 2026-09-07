use core::ops::Index;

use crate::{
    axis::{HardcodedCallerIndex, HardcodedCallerList},
    types::Address,
};

impl Index<HardcodedCallerIndex> for HardcodedCallerList {
    type Output = Address;

    fn index(&self, index: HardcodedCallerIndex) -> &Self::Output {
        &self.inner[index.inner]
    }
}
