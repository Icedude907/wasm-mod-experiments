#![allow(unused_parens)]

pub mod bindings_demo;
use crate::bindings_demo::*;

use rand::{SeedableRng, Rng};

// Exports a function termed "modmain" which the runtime can call.
#[no_mangle]
pub unsafe extern "C" fn modmain(){
    demo::host_fn();
    // Gets a random u64 from the host system. We are going to seed our own RNG for sending values back in this demonstration.
    let temp = demo::host_fn__u64();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(0);
    demo::host_fn_u32(rng.gen());
    demo::host_fn_u8_i16_u8__bool8(5, -21, 2);

    let temp2 = demo::host_fn__u32_bool8();

    let mut testptrbuf: [u8; 12] = [0;12];
    for n in 0..testptrbuf.len(){
        testptrbuf[n] = rng.gen();
    }
    demo::host_fn_u8x12p(&testptrbuf);

    demo::host_fn_strutf8("This is an example string - passed as a slice");
    demo::host_fn_strutf8(format!("Result from \'host_fn__u64\': {}", temp).as_str());
    demo::host_fn_strutf8(format!("Result from \'host_fn__u32_bool8\': {} {}", temp2.0, temp2.1).as_str());

    let buf = demo::host_fn__u64x24p();
    let xor = {
        let mut count = 0;
        for i in 0..buf.len(){
            count ^= buf[i];
        }
        count
    };
    demo::host_fn_strutf8(format!("Buffer stuff: 0x{:X}", xor).as_str());

    // Allocating a range of unknown size. (4-8mb)
    {
        let elements = (demo::host_fn__u64() % (1 << 19) + (1<<19)) as usize;
        let mut vector: Vec<u64> = vec![2; elements];
        let index = rng.gen_range(0..elements);
        vector[index] = 99;

        demo::host_fn();
    }
    /* WASM currently can't free memory. It'll stay at its max allocated amount.
        The rust-wasm compiler still includes an allocator for heap functions though.
    2 Solutions)
       Multiple memoryies:
        - Proposal allows for 1 wasm module to address multiple discrete blocks of memory.
        - Could arrange for a system where the module allocates a block of scratch memory - does all its big data operations, then asks the host to shrink it to 0.
        - The same system could be employed to expose host memory to the module, but functions would need to be continueously called to swap the data out with the stuff the guest requests.
        - TBH I quite dislike this proposal. It feels like patchwork and bloat. Such is the nature of the web ;)
       Virtual memory mapping:
        - Enables all sorts of memory manipulation - just like a regular .exe or whatnot (0-cost if using the host cpu's mmu).
        - Also allows for the host to map its memory into WASM space without copying with the potential to mark it as read/write only.
        - Likely stalled until 64 bit addressing implemented

       TBH this seems like an obvious flaw. creating one's own ecosystem to fix this though would be a crazy large undertaking
    // https://github.com/WebAssembly/design/issues/1397
    */

    // Allocating 6mb, may fit in the other
    {
        let elements = (1<<20)*6; // 16*6 = 96 pages
        let mut vector: Vec<u8> = vec![2; elements];
        let index = rng.gen_range(0..elements);
        vector[index] = 99;

        demo::host_fn_u32(vector[rng.gen_range(0..elements)] as u32);
        demo::host_fn();
    }

    // Alloc 3 = mem should be reused
    {
        let elements = (1<<20)*5; // 16*6 = 96 pages
        let mut vector: Vec<u8> = vec![2; elements];
        let index = rng.gen_range(0..elements);
        vector[index] = 99;

        demo::host_fn_u32(vector[rng.gen_range(0..elements)] as u32);
        demo::host_fn();
    }

    // Alloc using other function method
    let somedata = demo::host_fn__hostmemtest();
    demo::host_fn_strutf8(format!("Somedata: {} {} {} {}", somedata[5], somedata[31], somedata[99], somedata[420]).as_str());


}

// Thanks, https://stackoverflow.com/questions/69444896/how-to-pad-an-array-with-zeros
// This happens at runtime which it shouldn't. Hopefully will just work in the future as const fn support expands
// /*const*/fn pad_zeroes<const A: usize, const B: usize>(arr: [u8; A]) -> [u8; B] {
//     assert!(B >= A); //just for a nicer error message, adding #[track_caller] to the function may also be desirable
//     let mut b = [0; B];
//     b[..A].copy_from_slice(&arr);
//     b
// }