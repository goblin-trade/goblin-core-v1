const RATE_BYTES: usize = 136;

const RC: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808a,
    0x8000000080008000,
    0x000000000000808b,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008a,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000a,
    0x000000008000808b,
    0x800000000000008b,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800a,
    0x800000008000000a,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

const RHO: [u32; 25] = [
    0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56, 14,
];

const PI: [usize; 25] = [
    0, 10, 20, 5, 15, 16, 1, 11, 21, 6, 7, 17, 2, 12, 22, 23, 8, 18, 3, 13, 14, 24, 9, 19, 4,
];

const fn keccak_p(state: &mut [u64; 25]) {
    let mut round = 0;
    while round < 24 {
        // 1. Theta (θ)
        let mut c = [0u64; 5];
        let mut x = 0;
        while x < 5 {
            c[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
            x += 1;
        }

        let mut d = [0u64; 5];
        let mut x = 0;
        while x < 5 {
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            x += 1;
        }

        let mut i = 0;
        while i < 25 {
            state[i] ^= d[i % 5];
            i += 1;
        }

        // 2. Rho (ρ) and Pi (π)
        let mut b = [0u64; 25];
        let mut i = 0;
        while i < 25 {
            b[PI[i]] = state[i].rotate_left(RHO[i]);
            i += 1;
        }

        // 3. Chi (χ)
        let mut y = 0;
        while y < 5 {
            let mut x = 0;
            while x < 5 {
                state[x + 5 * y] =
                    b[x + 5 * y] ^ ((!b[(x + 1) % 5 + 5 * y]) & b[(x + 2) % 5 + 5 * y]);
                x += 1;
            }
            y += 1;
        }

        // 4. Iota (ι)
        state[0] ^= RC[round];

        round += 1;
    }
}

const fn absorb_block(state: &mut [u64; 25], block: &[u8; RATE_BYTES]) {
    let mut i = 0;
    while i < RATE_BYTES / 8 {
        let w = u64::from_le_bytes([
            block[i * 8],
            block[i * 8 + 1],
            block[i * 8 + 2],
            block[i * 8 + 3],
            block[i * 8 + 4],
            block[i * 8 + 5],
            block[i * 8 + 6],
            block[i * 8 + 7],
        ]);
        state[i] ^= w;
        i += 1;
    }
    keccak_p(state);
}

/// Compute Keccak-256 hash in const contexts at compile time
pub const fn const_keccak256(data: &[u8]) -> [u8; 32] {
    let mut state = [0u64; 25];
    let mut rate_buf = [0u8; RATE_BYTES];
    let mut buf_len = 0;

    let mut i = 0;
    while i < data.len() {
        rate_buf[buf_len] = data[i];
        buf_len += 1;
        if buf_len == RATE_BYTES {
            absorb_block(&mut state, &rate_buf);
            buf_len = 0;
        }
        i += 1;
    }

    // Pad: pad10*1 with domain 0x01
    rate_buf[buf_len] = 0x01;
    let mut j = buf_len + 1;
    while j < RATE_BYTES {
        rate_buf[j] = 0;
        j += 1;
    }
    rate_buf[RATE_BYTES - 1] ^= 0x80;

    absorb_block(&mut state, &rate_buf);

    let mut out = [0u8; 32];
    let mut word_idx = 0;
    while word_idx < 4 {
        let bytes = state[word_idx].to_le_bytes();
        let mut b = 0;
        while b < 8 {
            out[word_idx * 8 + b] = bytes[b];
            b += 1;
        }
        word_idx += 1;
    }
    out
}
