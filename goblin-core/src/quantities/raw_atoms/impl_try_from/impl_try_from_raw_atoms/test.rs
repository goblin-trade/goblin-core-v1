use crate::quantities::{RawAtoms, UnsidedAtoms};

#[test]
fn test_raw_atoms_to_atoms() {
    // Less than 6 decimals
    let raw = RawAtoms::<5>::from_u128(1_000_000);
    let atoms_result = UnsidedAtoms::try_from(raw);
    assert!(atoms_result.is_err());

    // USDC (6 decimals)
    let raw = RawAtoms::<6>::from_u128(1_000_000);
    let atoms = UnsidedAtoms::try_from(raw).unwrap();
    assert_eq!(atoms.inner, 1000000);

    // ETH (18 decimals)
    let raw = RawAtoms::<18>::from_u128(10u128.pow(18));
    let atoms = UnsidedAtoms::try_from(raw).unwrap();
    assert_eq!(atoms.inner, 1000000);
}

#[test]
fn test_dust() {
    // ETH (18 decimals)
    let raw = RawAtoms::<18>::from_u128(1_000_000);
    let atoms = UnsidedAtoms::try_from(raw).unwrap();
    assert_eq!(atoms.inner, 0);
}

#[test]
fn test_max_value() {
    // MAX possible value
    let raw = RawAtoms::<6>::MAX;
    let atoms = UnsidedAtoms::try_from(raw).unwrap();
    assert_eq!(atoms.inner, u64::MAX);

    // Set MSB (0 index in big endian) to 1
    let mut raw = RawAtoms::<6>::default();
    raw.0[0] = 1;
    let atoms = UnsidedAtoms::try_from(raw).unwrap();
    assert_eq!(atoms.inner, u64::MAX);

    // Smallest value greater than u128::MAX. This will get clamped to u128::MAX
    let mut raw = RawAtoms::<6>::default();
    raw.0[15] = 1;
    let atoms = UnsidedAtoms::try_from(raw).unwrap();
    assert_eq!(atoms.inner, u64::MAX);

    // Just below max value of u64
    let raw = RawAtoms::<6>::from_u128(u128::MAX - 1);
    let atoms = UnsidedAtoms::try_from(raw).unwrap();
    assert_eq!(atoms.inner, u64::MAX);

    // u64::MAX - 1
    let raw = RawAtoms::<6>::from_u128(u64::MAX as u128 - 1);
    let atoms = UnsidedAtoms::try_from(raw).unwrap();
    assert_eq!(atoms.inner, u64::MAX - 1);

    // Just below max value for ETH (18 decimals)
    let raw = RawAtoms::<18>::from_u128((u64::MAX as u128 - 1) * 10u128.pow(12));
    let atoms = UnsidedAtoms::try_from(raw).unwrap();
    assert_eq!(atoms.inner, u64::MAX - 1);
}
