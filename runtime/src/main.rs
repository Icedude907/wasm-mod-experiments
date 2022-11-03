#![allow(unused_parens)]
#![allow(unused_imports)] // TODO: Remove

use std::path::PathBuf;

use structopt::StructOpt;
use wasmer::{Store, Module, Instance, Function, Value, imports, ExportError::*, Memory, MemoryType, FunctionEnv};

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
    let mut store = Store::default();
    // A module that is run by the environment (the file). All functions exposed to this environment are passed to the module.
    let module = Module::new(&store, &module_binary).unwrap();

    // Wraps arbitrary data (modapi::ModEnv) we want to pass to functions called by wasm. Data is owned by the store.
    let fnenv = FunctionEnv::new(&mut store, modapi::ModEnvData{
        memory: unsafe{ std::mem::MaybeUninit::zeroed().assume_init() },
        counterfn_count: 0,
    });

    /* Defining all functions we expose + their handlers.
        Functions can be split between higher level groups ("game", "env") for destinction purposes. Useful with libraries.
        Integers (i,u,b) should all be passed and received as little endian as the wasm runtime operates on values in this manner.
        Though fundamentally the data you pass around is random bits, the wasm module has a few "types" (i32 i64 i8x8 etc). This is an odd decision.
    */
    let imports = imports!{
        // Functions that can be called to test wasm functionality
        "demo" => {
            "counter()"                 => Function::new_typed_with_env(&mut store, &fnenv, modapi::info_fn),
            "print(u32)"                => Function::new_typed(&mut store, modapi::print_u32),
            "rand() u64"                => Function::new_typed(&mut store, modapi::rand),
            "hostmath(u8 i16 u8) bool8" => Function::new_typed(&mut store, modapi::hostmath),
            "rand() (u32, bool8)"       => Function::new_typed(&mut store, modapi::rand_2),
            "print(ptr)"                => Function::new_typed_with_env(&mut store, &fnenv, modapi::print_buffer),
            "print(ptr, u32)"           => Function::new_typed_with_env(&mut store, &fnenv, modapi::print),
            "rand(ptr)"                 => Function::new_typed_with_env(&mut store, &fnenv, modapi::rand_buffer),
            "receive_big_buffer(ptr)"   => Function::new_typed_with_env(&mut store, &fnenv, modapi::send_big_buffer),
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
    let instance = Instance::new(&mut store, &module, &imports).unwrap(); // Creates an individual instance / vm for this module.
    {
        let mut env_mut = fnenv.as_mut(&mut store);
        env_mut.memory = instance.exports.get_memory("memory").unwrap().clone(); // Wire up the memory field to the actual module memory. (This is not automated becos of multiple-memories)
    }

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
    main_fn.call(&mut store, &[]).unwrap();

    if let Some(s) = optional_onshutdown_fn {
        println!("Running shutdown function.");
        s.call(&mut store, &[]).unwrap();
    }

    // Done with the demo. Time to print some stats TODO:
    println!("--------------------\n  Finished.");
    let mem = fnenv.as_ref(&store).memory.view(&store);
    println!("Memory used: {} ({} pages)", mem.data_size(), mem.size().0);

}
