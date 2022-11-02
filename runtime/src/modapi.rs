#![allow(non_snake_case)]
use wasmer::{WasmerEnv, LazyInit, Memory, WasmPtr, Array, MemoryView, ValueType};

/// wasmerenv setup. Allows for interfacing with associated data when a host function is called.
/// Data can be both user provided (e.g.: a name or uuid or whatnot), or passed by the "vm" (e.g.: memory).
/// Passed as a reference to *_with_env functions.
#[derive(WasmerEnv, Clone, Default)]
pub struct HostEnvForMods {
    // pub modname: String, // Arbitrary. Use however you need

    /*This will provide a reference to the "memory" section within the module.
      It doesn't exist until the environment starts (hence LazyInit)
      We transmute it to a 'Memory' type for convenient operation.
      https://docs.rs/wasmer/latest/wasmer/trait.WasmerEnv.html
      Effectively `instance.exports.get_memory("memory").unwrap();`
    */#[wasmer(export)]
    pub memory: LazyInit<Memory>,
}

/// Prints a line to the console. Some info is included
pub fn host_fn(env: &HostEnvForMods){
    let mem = env.memory_ref().unwrap();
    println!("[host]: Guest called void fn. Pages: {}", mem.size().0);
}
/// Prints the passed in number to the console. NOTE: Input will be little endian as per WASM spec
pub fn host_fn_u32(num: u32){
    let num = u32::from_le(num);
    println!("[host]: Guest called fn with arg u32: {}.", num);
}
/// Returns a random 64 bit number. NOTE: Out is Little Endian (but its seedless random so who cares?)
pub fn host_fn__u64() -> u64{
    let num = rand::random::<u64>();
    println!("[host]: Guest called fn. Returning random u64: {}", num);
    return num;
}

// NOTE: These arguments are sent oddly - passing as 3 i32s when it could be packed into 1.
// This is actually faster on the wasm side as no unpacking needs to take place, but slower on host / mem transfer fronts.
/// (num1+num2)%num3 == 0
pub fn host_fn_u8_i16_u8__bool8(num1: u8, num2: i16, num3: u8) -> u8{
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
pub fn host_fn__u32_bool8() -> (u32, u8){
    let ret = (rand::random::<u32>(), true as u8);
    println!("[host]: Guest called function. Returning tuple u32+bool8: ({}, {})", ret.0, ret.1 != 0);
    return ret;
}

#[derive(Copy, Clone)]
pub struct Arr<T, const N: usize>(pub [T; N]);
// No guarantee the other side will send a valid combination of values, hence its unsafe
unsafe impl<T: Copy, const N: usize> ValueType for Arr<T, N>{}

/// Prints a 12 byte byte buffer. Referenced via a pointer - requires access to memory
pub fn host_fn_u8x12p(env: &HostEnvForMods, ptr: WasmPtr<Arr<u8,12>>){
    // The WasmPtr system is very annoying. I was expecting to be able to put any kind of data type inside and have it work fine (its a pointer - it can be transmuted), but NOPE!
    // Instead either have to do WasmPtr<u8, Array> for arrays like [u8;12] (which has annoying api issues) or implement ValueType like I've done.
    // This is quite poor - instead I have to define a wrapper type. TODO: flesh it out properly

    // This converts to a real pointer - can fail if wasm gives a pointer which is/falls out of bounds
    let deref = ptr.deref(unsafe{env.memory_ref_unchecked()})
        .expect("The pointer given hits the end of memory!");
    // Annoying you can't modify the value in place using the api, you have to copy-edit-write.
    // TBH no idea why - they allow this behaviour with strings even tho it poses a synchronisation problem. FIXPLS
    let deref = deref.get().0;
    // let deref = unsafe{ deref.get_mut() };
    println!("[host]: Host received 12 chars of data from wasm address 0x{:X}: {:?}", ptr.offset(), deref);
}


/// Prints an arbirarily long string (or generic piece of data).
/// Make sure you aren't introducting a buffer overrun exploit somehow - this will depend on your unique scenario
// Unfortunately the current api doesnt allow recieving of arbitrary structs, even tho it should be simple?
pub fn host_fn_strutf8(env: &HostEnvForMods, ptr: WasmPtr<u8, Array>, len: u32){
    // This fun function does my job for me in this specific case - also verifies len is in bounds. How convenient.
    // Unsafe for the reason that it can be edited in the wasm environment (ref not copy)
    let deref = unsafe{ptr.get_utf8_str(env.memory_ref_unchecked(), len)};
    println!("[mod]: {}", deref.unwrap_or("I was going to say something - but the string I passed was invalid."));
    /* NOTE: To check bounds do something like:
        let end = ptr.offset.checked_add(str_len)?;
        if end as usize > memory.size().bytes().0 {
            return None;
        }
     */
}

/// As far as I know, the only practical way to insert bulk data into WASM is:
/// 1) Take in a pointer to a buffer, and fill out the data in there.
///     - Useful for medium sized data not returnable in a basic struct.
///     - Makes a discrete copy, no need to worry about corruption / modification on either side
/// Ideally there'd also be a 2nd way where you map host memory into the VM - but since wasm memory is linear this isn't a thing - yet.
pub fn host_fn__u64x24p(env: &HostEnvForMods, outbuf: WasmPtr<Arr<u64, 24>>){
    let mem = unsafe{outbuf.deref(env.memory_ref_unchecked())}.expect("Pointer provided was invalid");
    let mem = unsafe{ mem.get_mut() }; // TODO: Verify bounds FIXME
    for i in 0..24{
        mem.0[i] = rand::random::<u64>();
    }
}

/// This function aims to demonstrate method 2.
/// See [`host_fn__u64x24p`] for details
pub fn host_fn__hostmemtest(env: &HostEnvForMods, outbuf: WasmPtr<Arr<u8, 104857600>>) /*-> WasmPtr<u32>*/{
    println!("[host]: Long function called. Prepare to wait 10s+");
    let start = std::time::Instant::now();

    let mut data: Vec<u64> = Vec::with_capacity(13107200); // Rand optimisation. Could have used u8 but thats probs slower
    unsafe{ data.set_len(data.capacity()) };
    let mut data = data.into_boxed_slice();
    for i in 0..data.len(){
        data[i] = rand::random::<u64>();
    }

    println!("[host]: Generating ~100mb of u64 rand took {}s.", start.elapsed().as_millis() as f64 / 1000.0);
    let start = std::time::Instant::now();

    let mem = unsafe{outbuf.deref(env.memory_ref_unchecked())}.expect("Pointer provided was invalid"); // Location where data goes
    let mem = unsafe{ mem.get_mut() };
    let mem: &mut [u64; 13107200]  = unsafe{ std::mem::transmute(&mut mem.0) }; // Rand optimisation

    mem.copy_from_slice(&data[..]);

    println!("[host]: Sending over ~100mb took {}s.", start.elapsed().as_millis() as f64 / 1000.0);
}


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