pub mod flat_iterator;
pub use flat_iterator::*;

mod inner_bitmap_iter;
mod outer_bitmap_iter;
pub mod resting_order_iter;

use inner_bitmap_iter::*;
use outer_bitmap_iter::*;
