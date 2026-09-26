//! `take` integration tests.
//!
//! Cargo auto-discovers `tests/<dir>/main.rs` as its own test target, so every
//! module declared here is compiled into the `take` test binary. The shared
//! fixtures live at `tests/test_utils/mod.rs`; because this test binary has its
//! own directory the module has to be pulled in by path.
//!
//! Each test acquires `test_utils::isolated()` so the sibling modules do not
//! race on the process-global hostio and settlement state.

#[path = "../test_utils/mod.rs"]
mod test_utils;

mod test_take_order_book;
mod test_take_single_order;
