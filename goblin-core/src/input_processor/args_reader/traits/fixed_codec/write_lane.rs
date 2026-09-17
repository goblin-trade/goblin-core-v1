use crate::input_processor::ArgsWriter;

/// Write the low `byte_len` bytes of `lane`, little-endian, advancing `writer`.
pub fn write_lane(writer: &mut ArgsWriter, lane: u64, byte_len: usize) {
    match byte_len {
        1 => writer.write_bytes(&[lane as u8]),
        2 => writer.write_bytes(&(lane as u16).to_le_bytes()),
        4 => writer.write_bytes(&(lane as u32).to_le_bytes()),
        8 => writer.write_bytes(&lane.to_le_bytes()),
        // 3/5/6/7 (and 0): stage the bytes, then write them in one call.
        _ => {
            let mut buf = [0u8; 8];
            let mut i = 0;
            while i < byte_len {
                buf[i] = (lane >> (i * 8)) as u8;
                i += 1;
            }
            writer.write_bytes(&buf[..byte_len]);
        }
    }
}
