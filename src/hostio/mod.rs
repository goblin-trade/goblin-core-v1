mod hostio_unsafe;

pub mod hostio_buffer;
pub mod hostio_context;
pub mod hostio_helpers;

pub use hostio_buffer::*;
pub use hostio_context::*;
pub use hostio_helpers::*;

#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub mod indexer_hostio;
#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub use indexer_hostio::*;
