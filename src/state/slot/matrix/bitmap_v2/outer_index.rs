use crate::state::bitmap_v2::ordered_index::OrderedIndex;

pub type OuterIndex<I: OrderedIndex> = (<I::Prev as OrderedIndex>::Prev, I::Prev);
