pub trait ConstZero {
    const ZEROED: Self;
}

impl ConstZero for usize {
    const ZEROED: Self = 0;
}
