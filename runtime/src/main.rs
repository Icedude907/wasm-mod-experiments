#![allow(unused_parens)]

use std::path::PathBuf;

use structopt::StructOpt;
use wasmer::{Store, Module, Instance, Function, LazyInit, Value, imports};

pub mod modapi;

pub struct GameInstance{
    pub players_online: u64,
}

static mut GAME_INSTANCE: GameInstance = GameInstance{ players_online: 0, };
pub fn game_instance() -> &'static mut GameInstance{ // Not thread safe oh noes.
    unsafe{ &mut GAME_INSTANCE }
}

#[derive(StructOpt, Debug)]
struct Args{
    /// The module to run.
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Args::from_args();
    // println!("{:?}", args);

    // Read the file
    let module_binary = std::fs::read(args.path).unwrap();

    // A webassembly runtime environment
    let store = Store::default();
    // A module that is run by the environment (the file). All functions exposed to this environment are passed to the module.
    let module = Module::new(&store, &module_binary).unwrap();
    // I don't get why we have to go through all these loops to pass data about the wasm instance into a function. Surely a pointer would do?
    // Either way, this is a `wasmer` thing
    let memenv = modapi::MemoryAccessEnv{ memory: LazyInit::default() };
    // All functions we export to the function
    let import_object = imports!{
        "game" => {
            "void_fn()"
            "print_log(str)"            => Function::new_native_with_env(&store, memenv.clone(), modapi::print_log),
            "get_player_count: u64"     => Function::new_native(&store, modapi::get_player_count),
            "get_player(UUID): Player"  => Function::new_native_with_env(&store, memenv.clone(), modapi::get_player),
        },
    };
    // Preparing to run the module.
    let instance = Instance::new(&module, &import_object).unwrap();

    {   // Diagnostic
        println!("{:#?}", instance.exports);
    }

    // Run the module
    {
        let main_fn = instance.exports.get_function("modmain").unwrap();
        let shutdown_fn = instance.exports.get_function("shutdown").unwrap();
        let on_player_join_fn = instance.exports.get_function("on_player_join");

        main_fn.call(&[]).unwrap();

        let mut line = String::new();
        println!("The \"Game\" is running, type 'quit' to exit, and anything else to create a new 'player'.");
        loop{
            std::io::stdin().read_line(&mut line).unwrap();

            if(line.to_ascii_lowercase().contains("quit")){
                shutdown_fn.call(&[]).unwrap(); // Errors don't matter now.
                break;
            } // else

            // This would be good as a function. I think inspect() but it's an unstable feature.
            game_instance().players_online += 1;
            print!("  ... created.");
            match on_player_join_fn{
                Ok(f) => {
                    println!();
                    f.call(&[Value::I64(0x420faced)]).unwrap();
                }
                _ => { println!(" However, this mod doesn't handle this action."); }
            }
        }
    }

    // Print the output
    println!("Finished.");
}

pub struct Player{
    name: String,
    uuid: UUID,
}
type UUID = u64;