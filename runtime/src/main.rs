#![allow(unused_parens)]
#![allow(unused_imports)] // TODO: Remove

use std::path::PathBuf;

use structopt::StructOpt;
use wasmer::{Store, Module, Instance, Function, LazyInit, Value, imports, ExportError::*, Memory, MemoryType};

pub mod modapi;

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
    let store = Store::default();
    // A module that is run by the environment (the file). All functions exposed to this environment are passed to the module.
    let module = Module::new(&store, &module_binary).unwrap();
    // "for initializing the environments passed to host functions after instantiation but before execution."
    // "For example, exported items such as memories and functions which don’t exist prior to instantiation can be accessed here so that host functions can use them."
    let mut modenv = modapi::HostEnvForMods::default();

    /* Defining all functions we expose + their handlers.
        Functions can be split between higher level groups ("game", "env") for destinction purposes. Useful with libraries.
        I've also had to define my own calling convention - which I've essentially made a "pseudo function signature" with params:
            `name(arg1_t arg2_t) return_t`
        All values are passed as bits and are up for interpretation by the recipient, nevertheless this signature includes types for reader clarity.
        Integers (i,u,b) should all be passed and received as little endian as the wasm runtime operates on values in this manner.
    */
    let import_object = imports!{
        // Functions that can be called to test wasm functionality
        "demo" => {
            "counter()"                 => Function::new_native_with_env(&store, modenv.clone(), modapi::info_fn),
            "print(u32)"                => Function::new_native(&store, modapi::print_u32),
            "rand() u64"                => Function::new_native(&store, modapi::rand),
            "hostmath(u8 i16 u8) bool8" => Function::new_native(&store, modapi::hostmath),
            "rand() (u32, bool8)"       => Function::new_native(&store, modapi::rand_2),
            "print(ptr)"                => Function::new_native_with_env(&store, modenv.clone(), modapi::print_buffer),
            "print(ptr, u32)"           => Function::new_native_with_env(&store, modenv.clone(), modapi::print),
            "rand(ptr)"                 => Function::new_native_with_env(&store, modenv.clone(), modapi::rand_buffer),
            "receive_big_buffer(ptr)"   => Function::new_native_with_env(&store, modenv.clone(), modapi::send_big_buffer),
            // "prepare_arbitrary_string() u32"
            // "receive_arbitrary(ptr) enum8"
        },
        // Functions to manipulate the state of our "game"
        "game" => {
            // "print_log(str)"            => Function::new_native_with_env(&store, memenv.clone(), modapi::print_log),
            // "get_player_count: u64"     => Function::new_native(&store, modapi::get_player_count),
            // "get_player(UUID): Player"  => Function::new_native_with_env(&store, memenv.clone(), modapi::get_player),
        },
        // IDK some other function you want.
        "env" => {
        }
    };

    // Preparing to run the module.
    let instance = Instance::new(&module, &import_object).unwrap();

    // Diagnostic
    println!("{:#?}", instance.exports);

    /* At this point the WASMer environment is ready
        Problem: Our functions will never be called because the WASM environment isn't running right now
        Solution: Search the module for a preagreed upon function to call - which can then run your functions
        If the function on their end doesn't exist you handle that error (e.g.: Popup "module is invalid, the mod cannot start.")
            or, if it is non-essential you could run without it. (e.g.: Hooks and callbacks)
     */ // Inspect_err for in the future.
    let main_fn = instance.exports.get_function("modmain").unwrap();
    let optional_onshutdown_fn = instance.exports.get_function("onshutdown");
    if let Err(e) = &optional_onshutdown_fn{ // Preferred to use inspect_err: still nightly.
        match e {
            IncompatibleType => println!("Error: an exit function is defined with incompatible arguments!"),
            Missing(_) => println!("Note: no exit function is defined in the module.")
        }
    }
    let optional_onshutdown_fn = optional_onshutdown_fn.ok();

    println!("Running \"main\" function.");
    /* NOTE DONT DO THIS
       The WASM environment runs on the same thread as the caller
       If the module was to infinitely loop the program is going to hang
       Good practice would be on a separate thread with an optional timeout (or something async).
     */
    main_fn.call(&[]).unwrap();

    if let Some(s) = optional_onshutdown_fn {
        println!("Running shutdown function.");
        s.call(&[]).unwrap();
    }

    // Done with the demo. Time to print some stats TODO:
    println!("--------------------\n  Finished.");
    let mem = instance.exports.get_memory("memory").unwrap();
    println!("Memory used: {} ({} pages)", mem.data_size(), mem.size().0);

}
