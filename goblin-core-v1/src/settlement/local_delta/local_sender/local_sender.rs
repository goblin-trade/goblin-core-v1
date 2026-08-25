use goblin_macros::ConstDefault;

use crate::settlement::local_delta::{LocalMake, LocalTake};

#[derive(ConstDefault, Clone, Copy)]
pub struct LocalSender {
    pub take: LocalTake,
    pub make: LocalMake,
}
