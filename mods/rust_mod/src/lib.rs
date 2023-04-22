#![allow(unused_parens)]

mod bindings;
use crate::bindings::*;

use rand::{SeedableRng, Rng};

// Exports a function termed "modmain" which the runtime can call.
#[no_mangle]
pub unsafe extern "C" fn modmain(){
    demo::counter();
    demo::counter();
    // Gets a random u64 from the host system. We are going to seed our own RNG for sending values back in this demonstration.
    let temp = demo::rand();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(0);
    demo::print_u32(rng.gen());
    demo::hostmath(5, -21, 2);
    let temp2 = demo::rand_2();

    let mut testptrbuf: [u8; 12] = [0;12];
    for n in 0..testptrbuf.len(){
        testptrbuf[n] = rng.gen();
    }
    demo::print_buffer(&testptrbuf);

    demo::print("This is an example string - passed as a slice");
    demo::print(format!("Result from \'host_fn__u64\': {}", temp).as_str());
    demo::print(format!("Result from \'host_fn__u32_bool8\': {} {}", temp2.0, temp2.1).as_str());

    let buf = demo::rand_buffer();
    let xor = {
        let mut count = 0;
        for i in 0..buf.len(){
            count ^= buf[i];
        }
        count
    };
    demo::print(format!("Buffer stuff: 0x{:X}", xor).as_str());

    /* WASM currently can't free memory. It'll stay at its max allocated amount.
        The rust-wasm compiler still includes an allocator for heap functions though.
    2 Solutions)
       Multiple memoryies:
        - Proposal allows for 1 wasm module to address multiple discrete blocks of memory.
        - Could arrange for a system where the module allocates a block of scratch memory - does all its big data operations, then asks the host to shrink it to 0.
        - The same system could be employed to expose host memory to the module, but functions would need to be continueously called to swap the data out with the stuff the guest requests.
        - TBH I quite dislike this proposal. It feels like patchwork and bloat.
       Virtual memory mapping:
        - Enables all sorts of memory manipulation - just like a regular .exe or whatnot (near 0-cost if using the host cpu's mmu).
        - Also allows for the host to map its memory into WASM space without copying with the potential to mark it as read/write only.
        - Likely stalled until 64 bit addressing implemented

       TBH this seems like an obvious flaw. creating one's own ecosystem to fix this though would be a crazy large undertaking
    // https://github.com/WebAssembly/design/issues/1397
    */

    demo::counter();
    // Allocating a range of unknown size. (2-4mb)
    #[allow(non_upper_case_globals)]
    const megabyte: u32 = 1024*1024;
    {
        let elements = (2*megabyte) as usize;
        let mut vector: Vec<u64> = vec![2; elements];
        demo::counter();
    }

    // Recieving lots of data
    let somedata = demo::receive_big_buffer();
    demo::print(format!("Somedata: {} {} {} {}", somedata[5], somedata[31], somedata[99], somedata[420]).as_str());

    demo::print("I'm listening. Press q to exit");
    loop{
        let string = demo::receive_string();
        if(string.to_lowercase().contains("quit")){ break; }
        demo::print(format!("Fantastic! I got your message of: {}", string).as_str());
    }
    demo::print("Ah gottem ggs");


}