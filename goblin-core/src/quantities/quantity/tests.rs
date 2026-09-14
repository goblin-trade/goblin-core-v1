use crate::settlement::CheckedOps;

use super::*;

#[test]
fn test_ops() {
    let base_lots: BaseLots<u64> = Quantity::new(10);
    let base_units: BaseUnits<u64> = Quantity::new(5);
    let ticks: Ticks = Quantity::new(2);

    let more_ticks = Ticks::new(1);
    let _tick_sum = ticks + more_ticks;
    let _tick_diff = ticks - more_ticks;

    let mut mutable_ticks = ticks;
    mutable_ticks += more_ticks;
    mutable_ticks -= Ticks::new(1);

    let _lot_unit = base_lots * base_units;
    let _lot_unit_tick = _lot_unit * ticks;
    let _lot_per_unit = base_lots / base_units;

    let _checked_add = Ticks::new(1).checked_add(Ticks::new(2));
    let _checked_mul = base_lots.checked_mul(base_units);
}

#[test]
fn test_into_unsided() {
    let base_atoms = BaseAtoms::<u64>::new(1);
    base_atoms.unsided();

    let quote_atoms = QuoteAtoms::<u64>::new(1);
    quote_atoms.unsided();

    let _adjusted = AdjustedQuoteLots::new(1);
}

#[test]
fn test_min() {
    let a = BaseAtoms::<u64>::new(1);
    let b = BaseAtoms::<u64>::new(2);

    assert!(a.min(b) == a);
}

#[test]
fn test_u32_ops() {
    let base_lots: BaseLots<u32> = Quantity::new(10);
    let base_units: BaseUnits<u32> = Quantity::new(5);

    let _lot_unit = base_lots * base_units;
    let _lot_per_unit = base_lots / base_units;
    let _checked_add = base_lots.checked_add(base_lots);

    let widened: BaseLots<u64> = base_lots.into();
    assert!(widened == Quantity::new(10u64));
}

#[test]
fn test_i32_ops() {
    let delta: BaseAtoms<i32> = Quantity::new(-5);

    let widened: BaseAtoms<i64> = delta.into();
    assert!(widened == Quantity::new(-5i64));
}
