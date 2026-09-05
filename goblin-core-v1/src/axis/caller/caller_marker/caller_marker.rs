use crate::{
    axis::{CallerEnum, TokenMarker},
    axis_helpers::AxisMarker,
};

pub trait CallerMarker: AxisMarker<Enum = CallerEnum> {
    type CallerIndex;
}
