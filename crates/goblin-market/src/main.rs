#![cfg_attr(all(not(feature = "export-abi"), not(test)), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    goblin_market::main();
}
