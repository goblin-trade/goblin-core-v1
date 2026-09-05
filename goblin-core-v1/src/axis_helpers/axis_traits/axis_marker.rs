pub trait AxisMarker: Clone + Copy + PartialEq + PartialOrd + Default {
    /// The seed struct for this axis, e.g. `Leg`
    type Axis;
    /// The enum this marker maps back to, e.g. `LegEnum`
    type Enum: Copy + PartialEq;
    /// The concrete variant this marker type represents
    const VARIANT: Self::Enum;
}
