use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tiny_keccak::{Hasher, Keccak};

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output
}

fn format_bytes(bytes: &[u8]) -> String {
    let parts: Vec<String> = bytes.iter().map(|b| format!("0x{:02x}", b)).collect();
    format!("[{}]", parts.join(", "))
}

fn generate_eth_hashes(out_dir: &Path, callers: &[[u8; 20]]) {
    let dest_path = out_dir.join("eth_store_hashes.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(
        f,
        "pub const ETH_STORE_HASHES: [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLER_COUNT] = ["
    )
    .unwrap();
    for caller in callers {
        let mut preimage_bytes = Vec::with_capacity(21);
        preimage_bytes.push(2); // StorePreimage::<ETH>::SLOT_DISCRIMINATOR = 2 + 0 = 2
        preimage_bytes.extend_from_slice(caller);
        let hash = keccak256(&preimage_bytes);
        writeln!(f, "    SlotKey::new({}),", format_bytes(&hash)).unwrap();
    }
    writeln!(f, "];").unwrap();
}

fn generate_erc20_hashes(out_dir: &Path, callers: &[[u8; 20]], tokens: &[[u8; 20]]) {
    let dest_path = out_dir.join("hardcoded_erc20_store_hashes.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(
        f,
        "pub const HARDCODED_ERC20_STORE_HASHES: [[SlotKey<StorePreimage<HardcodedERC20>>; HARDCODED_ERC20_COUNT]; HARDCODED_CALLER_COUNT] = ["
    )
    .unwrap();
    for caller in callers {
        writeln!(f, "    [").unwrap();
        for token in tokens {
            let mut preimage_bytes = Vec::with_capacity(41);
            preimage_bytes.push(3); // StorePreimage::<HardcodedERC20>::SLOT_DISCRIMINATOR = 2 + 1 = 3
            preimage_bytes.extend_from_slice(caller);
            preimage_bytes.extend_from_slice(token);
            let hash = keccak256(&preimage_bytes);
            writeln!(f, "        SlotKey::new({}),", format_bytes(&hash)).unwrap();
        }
        writeln!(f, "    ],").unwrap();
    }
    writeln!(f, "];").unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    let callers: [[u8; 20]; 2] = [
        [0u8; 20],
        [
            0x3f, 0x1e, 0xae, 0x7d, 0x46, 0xd8, 0x8f, 0x08, 0xfc, 0x2f, 0x8e, 0xd2, 0x7f, 0xcb,
            0x2a, 0xb1, 0x83, 0xeb, 0x2d, 0x0e,
        ],
    ];

    let localnet_tokens: [[u8; 20]; 2] = [
        [
            0xe1, 0x08, 0x02, 0x24, 0xb6, 0x32, 0xa9, 0x39, 0x51, 0xa7, 0xcf, 0xa3, 0x3e, 0xee,
            0xa9, 0xfd, 0x81, 0x55, 0x8b, 0x5e,
        ],
        [
            0x3f, 0x1e, 0xae, 0x7d, 0x46, 0xd8, 0x8f, 0x08, 0xfc, 0x2f, 0x8e, 0xd2, 0x7f, 0xcb,
            0x2a, 0xb1, 0x83, 0xeb, 0x2d, 0x0e,
        ],
    ];

    generate_eth_hashes(out_path, &callers);
    generate_erc20_hashes(out_path, &callers, &localnet_tokens);
}
