#[cfg(all(not(test), not(target_arch = "wasm32")))]
extern "C" {
    pub fn index_deposit(recipient: *const u8, token: *const u8, lots: u64);
}
