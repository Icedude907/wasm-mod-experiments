#![allow(unused_parens)]

pub mod bindings;
use crate::bindings::*;

#[no_mangle]
pub unsafe extern "C" fn modmain(){
    print_log("Hello from the RustLang mod!");
}

#[no_mangle]
pub unsafe extern "C" fn shutdown(){
    let count = get_player_count();
    let mut msg = "Goodbye from the mod to".to_owned();
    if(count == 0){
        msg.push_str("... nobody?");
    }else{
        msg.push_str(" the ");
        msg.push_str(&count.to_string());
        msg.push_str(" player");
        if(count != 1){
            msg.push_str("s");
        }
        msg.push_str(" online!");
    }
    print_log(&msg[..]);
}

// Optional function
#[no_mangle]
pub unsafe extern "C" fn on_player_join(uuid: UUID){
    GLOBAL_STATE.uuids.push(uuid);

    let msg = format!("Note: new player with uuid '0x{:x}' joined. Keeping a copy of that.", uuid);
    print_log(&msg[..]);
}

struct State{
    uuids: Vec<UUID>
}
static mut GLOBAL_STATE: State = State{
    uuids: vec![],
};