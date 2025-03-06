///! Obtain init code from a .wasm file
///!
///! # Steps
///!
///! 1. ~~Convert the wasm to WAT, then back to WASM to remove dangling references.~~ Code
///! works even if this step is removed.
///! 2. Bortli compress the bytes
///! 3. Call contract_deployment_calldata to obtain deployment data from init_code.
///!
///! # Theory
///!
///! - The contract bytecode begins with prefix EFF00000. This prefix differentiates WASM
///! contracts from EVM contracts that use the prefix 6080604052
///! - The bytecode must be prepended with EVM opcodes so that the code is actually interpreted
///! as a contract. This is done by calling contract_deployment_calldata(). The init code
///! will begin with `7f00000000000000000000000000000000000000000000000000000000000004e58060`
///!
///! This script will take path to a file 'gobin_core.wasm' and output 'goblin_core.contract'
///! in the same folder. To deploy this file call
///!
///! ```sh
///! # Run script to generate goblin_core.contract
///! cargo run -p compile-contract --bin compile-contract
///!
///! cast send \
///!    --rpc-url http://127.0.0.1:8547 \
///!    --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
///!    --create 0x$(xxd -p goblin_core.contract | tr -d '\n')
///!
///! # Activate
///! cast send 0x0000000000000000000000000000000000000071 \
///!     "activateProgram(address)" 0xA6E41fFD769491a42A6e5Ce453259b93983a22EF \
///!     --rpc-url http://127.0.0.1:8547 \
///!     --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
///!     --value 0.0001ether
///! ```
///!
use alloy_primitives::U256;
use brotli2::read::BrotliEncoder;
use eyre::{eyre, Result, WrapErr};
use std::fs;
use std::io::Read;
use std::ops::Range;
use std::path::PathBuf;
use wasm_encoder::{Module, RawSection};
use wasmparser::{Parser, Payload};

const PROJECT_HASH_SECTION_NAME: &str = "project_hash";
const BROTLI_COMPRESSION_LEVEL: u32 = 11;
const EOF_PREFIX_NO_DICT: &str = "EFF00000";

// To run
//
// cargo run -p compile-contract --bin compile-contract
// Run button gives incorrect path
fn main() -> Result<()> {
    // Hardcoded path to WASM file - replace with your actual path
    let wasm_path = PathBuf::from("./target/wasm32-unknown-unknown/release/goblin_core.wasm");

    // Create a dummy project hash (all zeros in this example)
    let project_hash = [0u8; 32];

    // Compress the WASM file
    let (wasm, init_code) = compress_wasm(&wasm_path, project_hash)?;

    let deployment_data = contract_deployment_calldata(&init_code);

    // Write the contract code to a file
    let contract_output_path = wasm_path.with_extension("contract");
    fs::write(&contract_output_path, &deployment_data)?;
    println!(
        "Contract code written to: {}",
        contract_output_path.display()
    );

    // Print sizes for reference
    println!(
        "Original WASM size: {} bytes",
        fs::metadata(&wasm_path)?.len()
    );
    println!("Processed WASM size: {} bytes", wasm.len());
    println!("Contract code size: {} bytes", init_code.len());

    Ok(())
}

/// Reads a WASM file at a specified path and returns its brotli compressed bytes.
fn compress_wasm(wasm: &PathBuf, project_hash: [u8; 32]) -> Result<(Vec<u8>, Vec<u8>)> {
    let wasm = fs::read(wasm)?;
    let wasm = add_project_hash_to_wasm_file(&wasm, project_hash)?;
    let wasm = strip_user_metadata(&wasm)?;

    // Compress the WASM using Brotli
    let mut compressor = BrotliEncoder::new(&*wasm, BROTLI_COMPRESSION_LEVEL);
    let mut compressed_bytes = vec![];
    compressor
        .read_to_end(&mut compressed_bytes)
        .wrap_err("failed to compress WASM bytes")?;

    // Prepare the final contract code with the EOF prefix
    let mut contract_code = hex::decode(EOF_PREFIX_NO_DICT).unwrap();
    contract_code.extend(compressed_bytes);

    Ok((wasm.to_vec(), contract_code))
}

// Adds the hash of the project's source files to the wasm as a custom section
fn add_project_hash_to_wasm_file(
    wasm_file_bytes: &[u8],
    project_hash: [u8; 32],
) -> Result<Vec<u8>> {
    let section_exists = has_project_hash_section(wasm_file_bytes)?;
    if section_exists {
        println!("Wasm file bytes already contains a custom section with a project hash, not overwriting");
        return Ok(wasm_file_bytes.to_vec());
    }
    Ok(add_custom_section(wasm_file_bytes, project_hash))
}

// Checks if the WASM already has a project hash section
fn has_project_hash_section(wasm_file_bytes: &[u8]) -> Result<bool> {
    let parser = wasmparser::Parser::new(0);
    for payload in parser.parse_all(wasm_file_bytes) {
        if let wasmparser::Payload::CustomSection(reader) = payload? {
            if reader.name() == PROJECT_HASH_SECTION_NAME {
                println!(
                    "Found the project hash custom section name {}",
                    hex::encode(reader.data())
                );
                return Ok(true);
            }
        }
    }
    Ok(false)
}

// Adds a custom section to the WASM
fn add_custom_section(wasm_file_bytes: &[u8], project_hash: [u8; 32]) -> Vec<u8> {
    // Helper for adding a custom section
    fn write_custom_section(output: &mut Vec<u8>, name: &str, data: &[u8]) {
        // Custom section ID
        output.push(0);

        // Section size (name length + 1 for null terminator + data length)
        let section_size = name.len() + 1 + data.len();
        leb128::write::unsigned(output, section_size as u64).unwrap();

        // Name length as LEB128
        leb128::write::unsigned(output, name.len() as u64).unwrap();

        // Name bytes
        output.extend_from_slice(name.as_bytes());

        // Data
        output.extend_from_slice(data);
    }

    let mut bytes = vec![];
    bytes.extend_from_slice(wasm_file_bytes);
    write_custom_section(&mut bytes, PROJECT_HASH_SECTION_NAME, &project_hash);
    bytes
}

// Strips custom and unknown sections from the WASM
fn strip_user_metadata(wasm_file_bytes: &[u8]) -> Result<Vec<u8>> {
    let mut module = Module::new();
    // Parse the input WASM and iterate over the sections
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm_file_bytes) {
        match payload? {
            Payload::CustomSection { .. } => {
                // Skip custom sections to remove sensitive metadata
                println!("Stripped custom section from user wasm to remove any sensitive data");
            }
            Payload::UnknownSection { .. } => {
                // Skip unknown sections that might not be sensitive
                println!("Stripped unknown section from user wasm to remove any sensitive data");
            }
            item => {
                // Handle other sections as normal.
                if let Some(section) = item.as_section() {
                    let (id, range): (u8, Range<usize>) = section;
                    let data_slice = &wasm_file_bytes[range.start..range.end];
                    let raw_section = RawSection {
                        id,
                        data: data_slice,
                    };
                    module.section(&raw_section);
                }
            }
        }
    }
    // Return the stripped WASM binary
    Ok(module.finish())
}

/// Prepares an EVM bytecode prelude for contract creation.
pub fn contract_deployment_calldata(code: &[u8]) -> Vec<u8> {
    let code_len: [u8; 32] = U256::from(code.len()).to_be_bytes();
    let mut deploy: Vec<u8> = vec![];
    deploy.push(0x7f); // PUSH32
    deploy.extend(code_len);
    deploy.push(0x80); // DUP1
    deploy.push(0x60); // PUSH1
    deploy.push(42 + 1); // prelude + version
    deploy.push(0x60); // PUSH1
    deploy.push(0x00);
    deploy.push(0x39); // CODECOPY
    deploy.push(0x60); // PUSH1
    deploy.push(0x00);
    deploy.push(0xf3); // RETURN
    deploy.push(0x00); // version
    deploy.extend(code);
    deploy
}
