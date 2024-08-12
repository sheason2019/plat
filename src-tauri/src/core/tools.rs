pub fn to_wasm_params(s: &str) -> (u32, u32) {
    let ptr: *const u8 = s.as_ptr();
    (ptr as u32, s.len() as u32)
}
