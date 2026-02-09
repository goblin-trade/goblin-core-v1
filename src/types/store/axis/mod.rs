///! Axes or namespaces used by the exchange
///! Use them with `Marker` to generate sub types. The sub types will
///! then implement its respective trait.
pub mod leg;
pub mod market;
pub mod token;

pub use leg::*;
pub use market::*;
pub use token::*;
