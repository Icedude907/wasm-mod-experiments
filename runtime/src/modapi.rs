#![allow(non_snake_case)]
use std::{cell::RefCell, borrow::BorrowMut, io::Write};

use rand::Rng;
use wasmer::{Memory, WasmPtr, MemoryView, ValueType, FunctionEnv, FunctionEnvMut, WasmRef};


/// Passed to functions that want it. Allows the host to easily perform context sensitive actions. Can have multiple (up to a per function basis).
pub struct ModEnvData{
    pub memory: Memory, // This is never gonna be used until we have wired in the memory - so no reason to make it an optional.
    pub counterfn_count: u32,
    pub queued_arbitrary: Option<Vec<u8>>, // Rust doesn't have a *void. TODO: Queued per thread?
}
type ModEnvMut<'a> = FunctionEnvMut<'a, ModEnvData>; // Convenient alias

/// Prints a line to the console. Some info is included
pub fn info_fn(mut fnenv: ModEnvMut){
    let mem = fnenv.data().memory.view(&fnenv); // Oddly roundabout but it works?
    let memsize = mem.size().0;
    let mut env = fnenv.data_mut();
    println!("[host]: Guest called void fn. Pages: {}, times called: {}", memsize, env.counterfn_count);
    env.counterfn_count += 1;
}
/// Prints the passed in number to the console. NOTE: Input will be little endian as per WASM spec
pub fn print_u32(num: u32){
    let num = u32::from_le(num);
    println!("[host]: Guest called fn with arg u32: {}.", num);
}
/// Returns a random 64 bit number. NOTE: Out is Little Endian (but its seedless random so who cares?)
pub fn rand() -> u64{
    let num = rand::random::<u64>();
    println!("[host]: Guest called fn. Returning random u64: {}", num);
    return num;
}

// NOTE: These arguments are sent oddly - passing as 3 i32s when it could be packed into 1.
// This is actually faster on the wasm side as no unpacking needs to take place, but slower on host / mem transfer fronts.
/// (num1+num2)%num3 == 0
pub fn hostmath(num1: u8, num2: i16, num3: u8) -> u8{
    let num2 = i16::from_le(num2);
    // Sign extension works which is good.
    let math = (num1 as i32 + num2 as i32) % (num3 as i32) == 0;
    println!("[host]: Guest called function. ({}+{})%{} == 0: {}", num1, num2, num3, math);
    return math as u8; // Have to cast to pass
}

/* Be very cautious if aiming for multi language support (or even just multi verson support) when using types.
   Reordering + insertion of padding can make both sides interpret the data in very different ways.
   Wasmer automatically corrects C-styled return tuples (TODO: Confirm) on the runtime side.
   On the guest use #[repr(C)] to make rust mimic the barebones behaviour in C for consistent transfer.
 */
// #[repr(C)] struct a(u16, u8);
pub fn rand_2() -> (u32, u8){
    let ret = (rand::random::<u32>(), true as u8);
    println!("[host]: Guest called function. Returning tuple u32+bool8: ({}, {})", ret.0, ret.1 != 0);
    return ret;
}

/// Prints a 12 byte byte buffer. Referenced via a pointer - requires access to memory
pub fn print_buffer(fnenv: ModEnvMut, ptr: WasmPtr<[u8;12]>){
    let env = fnenv.data();
    let mem = env.memory.view(&fnenv);

    // This copies the data
    let data = ptr.read(&mem).expect("pointer in bounds");
    // let deref = unsafe{ deref.get_mut() };
    println!("[host]: Host received 12 chars of data from wasm address 0x{:X}: {:?}", ptr.offset(), data);
}

/// Prints an arbirarily long string (or generic piece of data).
// Make sure you aren't introducting a buffer overrun exploit somehow - this will depend on your unique scenario
pub fn print(fnenv: ModEnvMut, ptr: WasmPtr<u8>, len: u32){
    //                         ^ Had to unwrap struct ^ due to wasmer api limitation
    let env = fnenv.data();
    let mem = env.memory.view(&fnenv);
    // This fun function does my job for me in this specific case - also verifies len is in bounds. How convenient.
    // Unsafe for the reason that it can be edited in the wasm environment (ref not copy)
    let deref = ptr.read_utf8_string(&mem, len);
    println!("[mod]: {}", deref.unwrap_or("I was going to say something - but the string I passed was invalid.".to_string()));
    /* NOTE: To check bounds do something like:
        let end = ptr.offset.checked_add(str_len)?;
        if end as usize > memory.size().bytes().0 {
            return None;
        }
    */
}

/// Operates by creating a buffer with data, and copying it into WASM.
pub fn rand_buffer(fnenv: ModEnvMut, outbuf: WasmPtr<[u64;24]>){
    let env = fnenv.data();
    let mem = env.memory.view(&fnenv);

    let data: [u64;24] = rand::random();
    // The function will fail before writing if part/all of the data is to be placed out of bounds.
    // Either explode as here or use inspect_err or something.
    outbuf.deref(&mem).write(data).expect("pointer flows out of bounds.");
}

/// As far as I know, the only practical way to insert bulk data into WASM is:
/// Take in a pointer to a buffer, and fill out the data in there.
/// Ideally there'd also be a 2nd way where you map host memory into the VM - but since wasm memory is linear this isn't a thing - yet.
pub fn send_big_buffer(fnenv: ModEnvMut, outbuf: WasmPtr<[u8;104857600]>){
    let mem = fnenv.data().memory.view(&fnenv);

    println!("[host]: Long function called. Prepare to wait a bit generating random numbers");
    let start = std::time::Instant::now();

    // This code is scuffed because we can't easily directly allocate uninitialised data on the heap. (It is crazy to me how this somehow hasn't happened yet in rust)
    // We will soon - see nightly
    let mut data: Vec<u8> = Vec::with_capacity(104857600); // Rand optimisation. Could have used u8 but thats probs slower
    unsafe{ data.set_len(data.capacity()) };
    rand::RngCore::fill_bytes(&mut rand::thread_rng(), data.as_mut());

    println!("[host]: Generating ~100mb of u64 rand took {}s.", start.elapsed().as_millis() as f64 / 1000.0);
    let start = std::time::Instant::now();

    // NOTE: THIS METHOD IS MUCH SLOWER THAN IT USED TO BE, THANKS TO WASM 3.0.0's API SHORTCOMINGS.
    let outbuf: WasmPtr<u8> = unsafe{ std::mem::transmute(outbuf) };
    outbuf.slice(&mem, 104857600).expect("pointer flows out of bounds").write_slice(&data[..]).unwrap();
    // let mut dest = outbuf.deref(&mem).as_mut_ptr();
    // dest.copy_from_slice(&data[..]);

    println!("[host]: Sending over ~100mb took {}s.", start.elapsed().as_millis() as f64 / 1000.0);
}

/// Generic fn to send queued data
pub fn send_arbitrary(mut fnenv: ModEnvMut, outbuf: WasmPtr<u8>) -> u8{
    let env = fnenv.data();
    let mem = env.memory.view(&fnenv);
    let ret = match &env.queued_arbitrary {
        None => SendArbitraryResult::NotPrepared,
        Some(buf) => {
            let outbuf = outbuf.slice(&mem, buf.len() as u32);
            match outbuf {
                Err(_) => SendArbitraryResult::GenericFailure,
                Ok(outbuf) => {
                    outbuf.write_slice(&buf[..]).unwrap();
                    SendArbitraryResult::Sucess
                }, // Shouldn't concieveably fail - prev slic function ran the checks.
            }
        }
    };
    let env = fnenv.data_mut();
    env.queued_arbitrary = None;
    return ret as u8;
}
#[repr(u8)]
enum SendArbitraryResult{
    Sucess = 0,
    GenericFailure = 1,
    NotPrepared = 2,
}

pub fn prepare_arbitrary_string(mut fnenv: ModEnvMut) -> u32 { // TODO: Set to wasm usize
    // Data gathering
    print!("[host]: Hi the mod wants a string. pls type something ty:\n> "); let _ = std::io::stdout().flush();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.truncate(buf.trim_end().len());

    // Setup
    let len = buf.len();
    fnenv.data_mut().queued_arbitrary = Some(buf.into_bytes());
    return len as u32;
}

// trait Test<T>{
//     fn getTheThing() -> *const T;
// }
// impl<'a, T: ValueType> Test<T> for WasmRef<'a, T> {
//     fn getTheThing(self) -> *const T {
//         let base = self.buffer.base;
//     }
// }

/*============================================*\
|   NOW APPROACHING THE DUMPING GROUND         |
\*============================================*/

/*
Doesnt support 64 bit slices.
#[repr(C)]
#[derive(Copy, Clone)]
struct WasmSlice<T: Copy>{
    ptr: WasmPtr<T>,
    len: u32,
}
// I feel like I'm fighting the system as this is so roundabout
unsafe impl<T: Copy> ValueType for WasmSlice<T>{}
*/