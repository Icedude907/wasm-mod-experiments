#![allow(unused_parens)]

mod bindings;
use rand::{RngCore, SeedableRng};
use wasminterface::wasm_export;

#[wasm_export("modmain")]
fn main(){
    bindings::hello();
    bindings::print("Well, it appears this is working... welcome to the world of webassembly!");
    bindings::print(format!("u128 test: 0x{:032x}", bindings::recv128()).as_str());
    let mut randomnums = rand::rngs::SmallRng::seed_from_u64(bindings::rand64());
    let randomnums: [u64; 4] = std::array::from_fn(|_|randomnums.next_u64());
    let randomstring = randomnums.into_iter().map(|x|x.to_string()).collect::<Vec<_>>().join(", ");
    bindings::print(format!("Generating random numbers within the mod, seeded from the host.\n    [{}]", randomstring).as_str());
    panic!("I yearn for death")
}

#[wasm_export]
fn onshutdown(){
    bindings::printnumber(0);
}