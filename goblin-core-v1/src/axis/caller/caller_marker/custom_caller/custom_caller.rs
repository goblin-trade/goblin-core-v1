use crate::axis::{caller_marker::custom_caller::CustomCallerStub, CallerMarker, CustomCaller};

impl CallerMarker for CustomCaller {
    type CallerIndex = CustomCallerStub;
}
