pub trait TupleMarker {
    /// Data type representing pending deposit amount
    type Deposit: Clone + Copy + Default;
}
