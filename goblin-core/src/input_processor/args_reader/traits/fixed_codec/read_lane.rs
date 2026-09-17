use crate::input_processor::ArgsReader;

/// Read `byte_len` little-endian bytes into a `u64` lane, advancing `reader`.
///
/// Used by derived code for a struct with a top-level `#[codec(bits = N)]`,
/// where the whole struct is one bit stream whose field widths are only known
/// at compile time via `BitPack::CAPACITY`.
///
/// `byte_len` is a macro-generated constant, so the `match` folds away and the
/// power-of-two widths become a single unaligned load.
pub fn read_lane(reader: &ArgsReader, byte_len: usize) -> u64 {
    let o = reader.offset.get();
    let args = &reader.args;
    let lane = match byte_len {
        1 => args[o] as u64,
        2 => u16::from_le_bytes([args[o], args[o + 1]]) as u64,
        4 => u32::from_le_bytes([args[o], args[o + 1], args[o + 2], args[o + 3]]) as u64,
        8 => u64::from_le_bytes([
            args[o],
            args[o + 1],
            args[o + 2],
            args[o + 3],
            args[o + 4],
            args[o + 5],
            args[o + 6],
            args[o + 7],
        ]),
        // 3/5/6/7 (and 0): no native integer of that width, assemble bytewise.
        _ => {
            let mut lane = 0u64;
            let mut i = 0;
            while i < byte_len {
                lane |= (args[o + i] as u64) << (i * 8);
                i += 1;
            }
            lane
        }
    };
    reader.advance_offset(byte_len);
    lane
}
