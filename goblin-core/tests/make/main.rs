//! `make` integration tests.
//!
//! Cargo auto-discovers `tests/<dir>/main.rs` as its own test target, so every
//! module declared here is compiled into the `make` test binary. The shared
//! fixtures live at `tests/test_utils/mod.rs`; because this test binary has its
//! own directory the module has to be pulled in by path.

#[path = "../test_utils/mod.rs"]
mod test_utils;

mod test_make_across_outer_bitmaps;
mod test_make_multiple_orders;
mod test_make_single_order;
mod test_make_two_sided_book;
