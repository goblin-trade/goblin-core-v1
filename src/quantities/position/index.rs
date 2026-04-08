use crate::quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2};

pub trait Index: Clone + Copy + PartialEq {
    type Outer: Clone + Copy + PartialEq;
}

pub type OuterIndex<I: Index> = (<I::Outer as Index>::Outer, I::Outer);

impl Index for OuterBitmapIndexV2 {
    type Outer = ();
}

impl Index for OuterPosV2 {
    type Outer = OuterBitmapIndexV2;
}

impl Index for InnerPosV2 {
    type Outer = OuterPosV2;
}
