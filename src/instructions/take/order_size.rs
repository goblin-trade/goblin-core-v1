use crate::quantities::{AdjustedQuoteLots, BaseLots};

pub enum OrderSize {
    Bid(AdjustedQuoteLots),
    Ask(BaseLots),
}
