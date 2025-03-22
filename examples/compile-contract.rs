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
use eyre::{bail, eyre, Result, WrapErr};
use glob::glob;
use std::io::Read;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use tiny_keccak::{Hasher, Keccak};
use wasm_encoder::{Module, RawSection};
use wasmparser::{Parser, Payload};

const PROJECT_HASH_SECTION_NAME: &str = "project_hash";
pub const TOOLCHAIN_FILE_NAME: &str = "rust-toolchain.toml";
const BROTLI_COMPRESSION_LEVEL: u32 = 11;
const EOF_PREFIX_NO_DICT: &str = "EFF00000";

// To run
//
// cargo run -p compile-contract --bin compile-contract
// Run button gives incorrect path
fn main() -> Result<()> {
    // Hardcoded path to WASM file - replace with your actual path
    let wasm_path = PathBuf::from("./target/wasm32-unknown-unknown/release/goblin_core_v1.wasm");

    // Leave files vector empty to include all files in the project
    let project_hash = hash_files(vec![], OptLevel::S)?;

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

fn add_custom_section(wasm_file_bytes: &[u8], project_hash: [u8; 32]) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.extend_from_slice(wasm_file_bytes);
    wasm_gen::write_custom_section(&mut bytes, PROJECT_HASH_SECTION_NAME, &project_hash);
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

#[derive(Default, Clone, PartialEq)]
pub enum OptLevel {
    #[default]
    S,
    Z,
}

pub fn hash_files(source_file_patterns: Vec<String>, opt_level: OptLevel) -> Result<[u8; 32]> {
    let mut keccak = Keccak::v256();
    let mut cmd = Command::new("cargo");
    cmd.arg("--version");
    let output = cmd
        .output()
        .map_err(|e| eyre!("failed to execute cargo command: {e}"))?;
    if !output.status.success() {
        bail!("cargo version command failed");
    }
    keccak.update(&output.stdout);
    if opt_level == OptLevel::Z {
        keccak.update(&[0]);
    } else {
        keccak.update(&[1]);
    }

    let mut buf = vec![0u8; 0x100000];

    let mut hash_file = |filename: &Path| -> Result<()> {
        keccak.update(&(filename.as_os_str().len() as u64).to_be_bytes());
        keccak.update(filename.as_os_str().as_encoded_bytes());
        let mut file = std::fs::File::open(filename)
            .map_err(|e| eyre!("failed to open file {}: {e}", filename.display()))?;
        keccak.update(&file.metadata().unwrap().len().to_be_bytes());
        loop {
            let bytes_read = file
                .read(&mut buf)
                .map_err(|e| eyre!("Unable to read file {}: {e}", filename.display()))?;
            if bytes_read == 0 {
                break;
            }
            keccak.update(&buf[..bytes_read]);
        }
        Ok(())
    };

    // Fetch the Rust toolchain toml file from the project root. Assert that it exists and add it to the
    // files in the directory to hash.
    let toolchain_file_path = PathBuf::from(".").as_path().join(TOOLCHAIN_FILE_NAME);
    let _ = std::fs::metadata(&toolchain_file_path).wrap_err(
        "expected to find a rust-toolchain.toml file in project directory \
         to specify your Rust toolchain for reproducible verification",
    )?;

    let mut paths = all_paths(PathBuf::from(".").as_path(), source_file_patterns)?;
    paths.push(toolchain_file_path);
    paths.sort();

    for filename in paths.iter() {
        // println!(
        //     "File used for deployment hash: {}",
        //     filename.as_os_str().to_string_lossy()
        // );
        hash_file(filename)?;
    }

    let mut hash = [0u8; 32];
    keccak.finalize(&mut hash);
    println!(
        "project metadata hash computed on deployment: {:?}",
        hex::encode(hash)
    );
    Ok(hash)
}

fn all_paths(root_dir: &Path, source_file_patterns: Vec<String>) -> Result<Vec<PathBuf>> {
    let mut files = Vec::<PathBuf>::new();
    let mut directories = Vec::<PathBuf>::new();
    directories.push(root_dir.to_path_buf()); // Using `from` directly

    let glob_paths = expand_glob_patterns(source_file_patterns)?;

    while let Some(dir) = directories.pop() {
        for entry in fs::read_dir(&dir)
            .map_err(|e| eyre!("Unable to read directory {}: {e}", dir.display()))?
        {
            let entry = entry.map_err(|e| eyre!("Error finding file in {}: {e}", dir.display()))?;
            let path = entry.path();

            if path.is_dir() {
                if path.ends_with("target") || path.ends_with(".git") {
                    continue; // Skip "target" and ".git" directories
                }
                directories.push(path);
            } else if path.file_name().map_or(false, |f| {
                // If the user has has specified a list of source file patterns, check if the file
                // matches the pattern.
                if !glob_paths.is_empty() {
                    for glob_path in glob_paths.iter() {
                        if glob_path == &path {
                            return true;
                        }
                    }
                    false
                } else {
                    // Otherwise, by default include all rust files, Cargo.toml and Cargo.lock files.
                    f == "Cargo.toml" || f == "Cargo.lock" || f.to_string_lossy().ends_with(".rs")
                }
            }) {
                files.push(path);
            }
        }
    }
    Ok(files)
}

fn expand_glob_patterns(patterns: Vec<String>) -> Result<Vec<PathBuf>> {
    let mut files_to_include = Vec::new();
    for pattern in patterns {
        let paths = glob(&pattern)
            .map_err(|e| eyre!("Failed to read glob pattern '{}': {}", pattern, e))?;
        for path_result in paths {
            let path = path_result.map_err(|e| eyre!("Error processing path: {}", e))?;
            files_to_include.push(path);
        }
    }
    Ok(files_to_include)
}
