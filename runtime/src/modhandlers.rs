use std::io::Write;

use rand::{rngs::SmallRng, RngCore, SeedableRng};
use wasmer::{FunctionEnvMut, Memory, MemoryView, WasmPtr, WasmSlice};

type WasmStateView<'a> = FunctionEnvMut<'a, ModWasmState>;
pub struct ModWasmState{
    pub memory_mod: Option<Memory>, // WASM linear memory block (automatically grows but cannot shrink - language flaw?)
    pub memory_sendbuf: Option<Vec<u8>>,

    pub fncounter_count: i32,
    pub rand: SmallRng,
}
impl ModWasmState{
    pub fn new()->Self{ Self {
        memory_mod: None, // Assigned after the instance created
        memory_sendbuf: None,
        fncounter_count: 0,
        rand: SmallRng::seed_from_u64(rand::thread_rng().next_u64()),
    } }
}
// Cut on bloat by adding methods to the types
pub trait ViewHelper {
    fn view_mem(&self) -> MemoryView<'_>;
}
impl ViewHelper for WasmStateView<'_>{
    fn view_mem(&self) -> MemoryView<'_> {
        return self.data().memory_mod.as_ref().unwrap().view(&*self);
    }
}
trait PointerSlicer{
    fn slice(&self, ptr: WasmPtr<u8>, len: u32) -> Result<WasmSlice<'_, u8>, ()>;
}
impl PointerSlicer for MemoryView<'_>{
    fn slice(&self, ptr: WasmPtr<u8>, len: u32) -> Result<WasmSlice<'_, u8>, ()> {
        return ptr.slice(self, len).map_err(|_|());
    }
}

// Compatibility wrappers
#[allow(non_snake_case)]
mod ResultWithLength{
    pub fn new(result: u32, len: u32) -> u64{
        return (result as u64) << 32 | len as u64;
    }
}
#[allow(non_camel_case_types)]
type bool32 = u32;

//--------------------------
// Mod API functions below
//--------------------------

pub fn hello(){
    println!("[mod]: Hello!");
}

pub fn fncounter(mut env: WasmStateView){
    let env = env.data_mut();
    env.fncounter_count += 1;
    println!("[mod]: Called `fncounter` (count = {})", env.fncounter_count);
}

pub fn printnumber(num: u32){
    println!("[mod]: Number {}", num);
}

pub fn rand64(mut env: WasmStateView) -> u64 {
    let env = env.data_mut();
    return env.rand.next_u64();
}

pub fn recv128(env: WasmStateView, ptr: WasmPtr<u128>){
    let view = env.view_mem();
    let _ = ptr.write(&view, 0x001122334455667788_99aabbccddeeff);
}

pub fn print(env: WasmStateView, ptr: WasmPtr<u8>, len: u32){
    let slice = env.view_mem(); // I don't understand why rust needs me to separate the binding out if it knows that's the only way to make the code compile.
    let Ok(slice) = slice.slice(ptr, len) else { return };
    // I'm just trying to read bytes one at a time and escape the non-ascii bits.
    let mut sb = String::new();
    for b in slice.iter(){
        let b = b.read().unwrap();
        match b{
            b'\n' => {
                println!("[mod]: \"{}\"", sb);
                sb = String::new();
            },
            b'"' => sb.push_str("\""),
            0x20..=0x7E => sb.push(b as char),
            _ => sb.push_str(format!("\\x{:02X}", b).as_str()),
        }
    }
    println!("[mod]: \"{}\"", sb);
}

pub fn getline(mut env: WasmStateView) -> u64{
    print!("[host]: Mod wants a string. Pls type something ty:\n> ");
    let _ = std::io::stdout().flush();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.truncate(buf.trim_end().len());

    // Write the response into the secondary memory
    let len = buf.len();
    env.data_mut().memory_sendbuf = Some(buf.into_bytes());

    // Formulate return value
    return ResultWithLength::new(true as u32, len as u32);
}

pub fn bulkdump(mut env: WasmStateView, target: WasmPtr<u8>) -> bool32{
    let Some(bulk) = env.data_mut().memory_sendbuf.take() else {
        return false as _;
    };
    let slice = env.view_mem();
    return slice.write(target.offset() as _, &bulk).is_ok() as _;
}