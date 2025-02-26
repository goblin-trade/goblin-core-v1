use brotli2::read::BrotliEncoder;
use eyre::{bail, eyre, Result, WrapErr};
use std::fs;
use std::io::Read;
use std::ops::Range;
use std::path::{Path, PathBuf};
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

    println!("Processing WASM file: {}", wasm_path.display());

    // Compress the WASM file
    let (wasm, contract_code) = compress_wasm(&wasm_path, project_hash)?;

    // TODO we should
    // - Add init bytecode
    // - Store as hex, `xxd -p goblin_core.contract | tr -d '\n' > goblin_core_hex`

    // Write the processed WASM to a file
    let wasm_output_path = wasm_path.with_extension("processed.wasm");
    fs::write(&wasm_output_path, &wasm)?;
    println!("Processed WASM written to: {}", wasm_output_path.display());

    // Write the contract code to a file
    let contract_output_path = wasm_path.with_extension("contract");
    fs::write(&contract_output_path, &contract_code)?;
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
    println!("Contract code size: {} bytes", contract_code.len());

    Ok(())
}

/// Reads a WASM file at a specified path and returns its brotli compressed bytes.
fn compress_wasm(wasm: &PathBuf, project_hash: [u8; 32]) -> Result<(Vec<u8>, Vec<u8>)> {
    let wasm =
        fs::read(wasm).wrap_err_with(|| eyre!("failed to read Wasm {}", wasm.to_string_lossy()))?;

    // Convert the WASM from binary to text and back to binary
    // This removes any dangling mentions of reference types
    let wat_str =
        wasmprinter::print_bytes(&wasm).map_err(|e| eyre!("failed to convert Wasm to Wat: {e}"))?;

    let wasm =
        wat2wasm(wat_str.as_bytes()).map_err(|e| eyre!("failed to convert Wat to Wasm: {e}"))?;

    // Add the project's hash as a custom section
    let wasm = add_project_hash_to_wasm_file(&wasm, project_hash)
        .wrap_err("failed to add project hash to wasm file as custom section")?;

    // Strip user metadata
    let wasm =
        strip_user_metadata(&wasm).wrap_err("failed to strip user metadata from wasm file")?;

    // Parse the WASM again to ensure it's valid
    let wasm = wat2wasm(&wasm).wrap_err("failed to parse Wasm")?;

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

// Simple wrapper around the wat2wasm functionality
fn wat2wasm(wat: &[u8]) -> Result<Vec<u8>> {
    wat::parse_bytes(wat)
        .map(|cow| cow.into_owned())
        .map_err(|e| eyre!("Failed to parse WAT: {}", e))
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
