#![allow(unused_parens)]

use std::path::PathBuf;

use structopt::StructOpt;
use wasmer::{Store, Module, Instance, Function, imports, ExportError::*, FunctionEnv};

pub mod modhandlers;

#[derive(StructOpt, Debug)]
struct Args{
    /// The module to run.
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Args::from_args();
    let module_binary = std::fs::read(args.path).unwrap();

    // A webassembly runtime environment
    let mut store = Store::default();
    // A module that is run by the environment (the file). All functions exposed to this environment are passed to the module.
    let module = Module::new(&store, &module_binary).unwrap();

    // Wraps arbitrary data (modapi::ModEnv) we want to pass to functions called by wasm. Data is owned by the store.
    let modenv = FunctionEnv::new(&mut store, modhandlers::ModWasmState::new());

    /* All functions the host exposes.
        Fundamentally we are passing random bits, but wasm expresses this in "types" (f32, i64, etc).
        I find this an odd decision (that is, why is memory typed? Surely only operations should be?)
    */
    let imports = imports!{
        "game" => { // Wasm exports are namespaced
            // Print a hello message (void function test)
            "hello"     => Function::new_typed(&mut store, modhandlers::hello),
            // Prints a counter with how many times the function is called (host-state function test)
            "fncounter"     => Function::new_typed_with_env(&mut store, &modenv, modhandlers::fncounter),
            // Print the provided number (single argument test)
            "printnumber"   => Function::new_typed(&mut store, modhandlers::printnumber),
            // Receive a 64 bit random number (i64 return type)
            "rand64"    => Function::new_typed_with_env(&mut store, &modenv, modhandlers::rand64),
            // Receive a 128-bit number (return via a pointer - the multi-value proposal is not supported by any calling convention)
            "recv128"   => Function::new_typed_with_env(&mut store, &modenv, modhandlers::recv128),
            // Print the provided string slice (multi-parameter test, pointer+length)
            "print"     => Function::new_typed_with_env(&mut store, &modenv, modhandlers::print),
            // Request user input from the console. Blocks until user submits.
            // Returns an i64-packed tuple. The upper 32 bits contain a success boolean, the lower 32 bits contain the length of data.
            // Upon returning true, the `bulkrecv` function can be used to write those bytes into a block of memory.
            // NOTE: Multiple-memories are not supported anywhere, so we must resort to a two-step process to provide the buffer to the target.
            "getline"   => Function::new_typed_with_env(&mut store, &modenv, modhandlers::getline),
            // Dumps the enqueued buffer (e.g.: from `getline`) into the specified memory location.
            // If there was no enqueued buffer, returns u32(false). Returns true if the buffer was written out.
            // The enqueued buffer is cleared in the host's memory after the call.
            "bulkdump"  => Function::new_typed_with_env(&mut store, &modenv, modhandlers::bulkdump),
        }
    };

    // Preparing to run the module.
    let instance = Instance::new(&mut store, &module, &imports).unwrap();
    // Wire up the memory field to the actual module memory. (This is not automated becos of multiple-memories)
    modenv.as_mut(&mut store).memory_mod = Some(instance.exports.get_memory("memory").unwrap().clone());

    // Diagnostic
    println!("Wasm module exports: {:#?}", instance.exports);

    // We are ready to run the wasm, but what do we run?
    // Search for a pre-agreed upon function which we labelled as "modmain()->()".
    // Else we just scream and die.
    const NAME_MAIN: &str = "modmain";
    const NAME_SHUTDOWN: &str = "onshutdown";
    let main_fn = instance.exports.get_function(NAME_MAIN).unwrap();
    // Optional callback function
    let optional_onshutdown_fn = instance.exports.get_function(NAME_SHUTDOWN).inspect_err(|e| match e {
        IncompatibleType => println!("Error: {} is not a function!", NAME_SHUTDOWN),
        Missing(_) => println!("Note: {} is not defined.", NAME_SHUTDOWN)
    }).ok();

    // NOTE: probably DONT DO THIS
    // If the module is malicious it can do an infinite loop and hang the main thread.
    // Run it asynchronously or impose a time limit or something.
    println!("Running \"{}\":", NAME_MAIN);
    let result = main_fn.call(&mut store, &[]);
    match result{
        Ok(_) => {},
        Err(_) => { println!("Function errored out. Did the mod crash?") }
    }

    // If one function crashes the VM, the other functions can still be used.
    // (Although this is probably not smart - module memory may be corrupted)
    if let Some(s) = optional_onshutdown_fn {
        println!("Running {}:", NAME_SHUTDOWN);
        let _ = s.call(&mut store, &[]); // Non-critical, discard result
    }

    // Done with the demo. Time to print some stats.
    println!("--------------------\n  Finished.");
    let mem = modenv.as_ref(&store).memory_mod.as_ref().unwrap().view(&store);
    println!("Memory used: {}b ({} pages)", mem.data_size(), mem.size().0);

}
