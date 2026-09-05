use crate::{
    axis::{CallerMarker, HardcodedCaller, HardcodedCallerIndex},
    types::Address,
};

impl CallerMarker for HardcodedCaller {
    type CallerIndex = HardcodedCallerIndex;
}
