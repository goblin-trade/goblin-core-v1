pub mod local_make;
pub mod local_sender_update;
pub mod local_take;

pub use local_make::*;
pub use local_sender_update::*;
pub use local_take::*;

use goblin_macros::ConstDefault;

#[derive(ConstDefault, Clone, Copy)]
pub struct LocalSender {
    pub take: LocalTake,
    pub make: LocalMake,
}
