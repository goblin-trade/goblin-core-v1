pub trait ConstZero {
    const ZEROED: Self;
}

impl ConstZero for () {
    const ZEROED: Self = ();
}
