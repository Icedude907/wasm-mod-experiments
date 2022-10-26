use wasmer::{WasmerEnv, LazyInit, Memory, WasmPtr, Array, MemoryView};

use crate::*;

#[derive(Clone, WasmerEnv)]
pub struct MemoryAccessEnv{
    #[wasmer(export)]
    pub(crate) memory: LazyInit<Memory>
}
impl MemoryAccessEnv{
    fn memory<'a>(&'a self) -> &'a Memory{
        return self.memory_ref().unwrap();
    }
}

pub fn print_log(mem: &MemoryAccessEnv, ptr: WasmPtr<u8, Array>, len: u32){
    // ctx is current instance
    let mem = mem.memory();
    // Perfectly safe. Execution of the runtime halts while calling functions (except in multithreaded modes, in which case it's the guest's fault),
    //  and we don't reference this memory after the guest could have modified it.
    // Tho... can this be used to address out of bounds?
    unsafe{
        let ptr = mem.data_ptr().add(ptr.offset() as usize) as *const u8;
        let slice = std::slice::from_raw_parts(ptr, len as usize);
        let slice = std::str::from_utf8_unchecked(slice);

        println!("[mod_LOG]: {}", slice);
    }
}

pub fn get_player_count() -> u64{
    return game_instance().players_online;
}

// Gets a player's data given a UUID.
pub fn get_player(mem: &MemoryAccessEnv, uuid: UUID) -> WasmPtr<mod_t::Player>{
    // This function works by allocating memory within the WASM sandbox and returning a pointer to that.
    // By doing so, the other side can read it as an owned string to be resized among other things.
    let memory_view: MemoryView<u8> = mem.memory().view();
    // let str = ptr.read_utf8_string_unchecked(&memory_view, length as u32).unwrap();
    // println!("Memory contents: {:?}", str);

    let p = Player{name: "Jimothy".to_string(), uuid: 0x0};
    // TODO: Insert into mem
    
    return WasmPtr::new(0);
}

// Types that are modified for ABI communication
pub mod mod_t{
    use wasmer::{WasmPtr, Array};

    #[derive(Clone, Copy)]
    pub struct Player{
        name_ptr: WasmPtr<u8, Array>,
        name_len: u32,
        uuid: crate::UUID,
    }
}