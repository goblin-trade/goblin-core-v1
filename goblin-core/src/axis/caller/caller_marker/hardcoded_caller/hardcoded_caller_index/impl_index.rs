use core::ops::Index;

use super::HardcodedCallerIndex;
use crate::{axis::caller::HardcodedCallerList, types::Address};

impl Index<HardcodedCallerIndex> for HardcodedCallerList {
    type Output = Address;

    fn index(&self, index: HardcodedCallerIndex) -> &Self::Output {
        &self.inner[index.inner]
    }
}
