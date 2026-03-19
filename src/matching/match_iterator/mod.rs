pub mod match_iterator;
pub use match_iterator::*;

mod inner_bitmap_iter;
mod outer_bitmap_iter;
mod resting_order_iter;

use inner_bitmap_iter::*;
use outer_bitmap_iter::*;
use resting_order_iter::*;
