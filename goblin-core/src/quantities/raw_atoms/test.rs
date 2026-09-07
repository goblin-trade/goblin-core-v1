use crate::quantities::RawAtoms;

#[test]
fn test_zero_raw_atoms() {
    let raw_atoms = RawAtoms::<8>([0u8; 32]);
    assert_eq!(raw_atoms.to_clamped_u128(), 0);
}

#[test]
fn test_small_value() {
    let mut raw_atoms = RawAtoms::<8>([0u8; 32]);
    raw_atoms.0[31] = 1;
    assert_eq!(raw_atoms.to_clamped_u128(), 1);

    let mut raw_atoms = RawAtoms::<8>([0u8; 32]);
    let expected_value = 100u128;
    let expected_bytes = expected_value.to_be_bytes();
    raw_atoms.0[16..].copy_from_slice(&expected_bytes);
    assert_eq!(raw_atoms.to_clamped_u128(), expected_value);
}

#[test]
fn test_clamping() {
    let mut raw_atoms = RawAtoms::<8>([0u8; 32]);
    raw_atoms.0[15] = 1;
    assert_eq!(raw_atoms.to_clamped_u128(), u128::MAX);
}
