use crate::quantities::{RawAtoms, UnsidedAtoms};

#[test]
fn test_atoms_to_raw_atoms() {
    // atoms will convert to raw atoms without any loss of data
    // We just need to ensure that decimal places are valid

    let atoms = UnsidedAtoms::new(1);

    // Less than 6 decimals
    assert!(RawAtoms::<5>::try_from(atoms).is_err());

    // More than 18 decimals
    assert!(RawAtoms::<19>::try_from(atoms).is_err());

    // 6 decimals
    let raw_atoms = RawAtoms::<6>::try_from(atoms).unwrap();
    assert_eq!(raw_atoms.to_clamped_u128(), 1);

    // 7 decimals
    let raw_atoms = RawAtoms::<7>::try_from(atoms).unwrap();
    assert_eq!(raw_atoms.to_clamped_u128(), 10);

    // 18 decimals
    let raw_atoms = RawAtoms::<18>::try_from(atoms).unwrap();
    assert_eq!(raw_atoms.to_clamped_u128(), 10u128.pow(12));
}

#[test]
fn test_max_atoms() {
    let atoms = UnsidedAtoms::new(u64::MAX);

    // 6 decimals
    let raw_atoms = RawAtoms::<6>::try_from(atoms).unwrap();
    assert_eq!(raw_atoms.to_clamped_u128(), u64::MAX as u128);

    // 7 decimals
    let raw_atoms = RawAtoms::<7>::try_from(atoms).unwrap();
    assert_eq!(raw_atoms.to_clamped_u128(), u64::MAX as u128 * 10);

    // 18 decimals
    let raw_atoms = RawAtoms::<18>::try_from(atoms).unwrap();
    assert_eq!(
        raw_atoms.to_clamped_u128(),
        u64::MAX as u128 * 10u128.pow(12)
    );
}
