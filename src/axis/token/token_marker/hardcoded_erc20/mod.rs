pub mod hardcoded_erc20;
pub mod hardcoded_erc20_data;
pub mod hardcoded_erc20_index;
pub mod hardcoded_tokens;

pub use hardcoded_erc20::*;
pub use hardcoded_erc20_data::*;
pub use hardcoded_erc20_index::*;
pub use hardcoded_tokens::*;

mod decode;

#[cfg(feature = "localnet")]
pub mod localnet;

#[cfg(feature = "mainnet")]
pub mod mainnet;
#[cfg(feature = "testnet")]
pub mod testnet;

#[cfg(feature = "localnet")]
pub use localnet::*;
#[cfg(feature = "mainnet")]
pub use mainnet::*;
#[cfg(feature = "testnet")]
pub use testnet::*;
