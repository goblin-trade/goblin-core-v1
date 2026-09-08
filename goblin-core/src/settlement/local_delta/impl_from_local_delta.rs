use crate::settlement::{ConstDefault, LocalCounterparties, LocalDelta, LocalSender};

impl<'a> From<&'a mut LocalCounterparties> for LocalDelta<'a> {
    fn from(value: &'a mut LocalCounterparties) -> Self {
        Self::new(LocalSender::DEFAULT, value)
    }
}
