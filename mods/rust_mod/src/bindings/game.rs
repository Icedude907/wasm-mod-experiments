#![allow(non_snake_case)]
#![allow(dead_code)]
// These bindings were written by hand.

// Sadly, we need to unwrap the unsafety all over the show
pub fn hello(){unsafe{ externs::hello() }}
pub fn fncounter(){unsafe{ externs::fncounter() }}
pub fn printnumber(num: u32){unsafe{ externs::printnumber(num) }}
pub fn rand64()->u64{unsafe{ externs::rand64() }}
pub fn recv128()->u128{unsafe{ externs::recv128() }}
pub fn print(str: &str){
    unsafe{ externs::print(str.as_ptr(), str.len() as u32) }
}
pub fn getline()->Option<String>{
    let result = unsafe{ externs::getline() };
    let (ok, len) = (result >> 32 != 0, result as u32);
    if !ok { return None; }
    let mut str = String::with_capacity(len as _);
    unsafe{ externs::bulkdump(str.as_mut_ptr()); }
    return Some(str);
}

// Underlying unsafe functions
mod externs{
    use wasminterface::wasm_import;

    #[wasm_import(, "game")] pub fn hello();
    #[wasm_import(, "game")] pub fn fncounter();
    #[wasm_import(, "game")] pub fn printnumber(num: u32);
    #[wasm_import(, "game")] pub fn rand64() -> u64;
    #[wasm_import(, "game")] pub fn recv128() -> u128; // rust directly complies with the C abi on u128
    #[wasm_import(, "game")] pub fn print(src: *const u8, len: u32);
    #[wasm_import(, "game")] pub fn getline() -> u64;
    #[wasm_import(, "game")] pub fn bulkdump(dst: *mut u8) -> bool;
}