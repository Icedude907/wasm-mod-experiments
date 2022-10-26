// These bindings were written by hand.
// Ideally in the future this would be fully automated, but as it stands the tools avaliable are not fit for my purposes.

// pub mod bindings{
    // Underlying functions
    mod externs{
        #[link(wasm_import_module = "game")]
        extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "print_log(str)")]
            pub fn print_log(ptr: u32, len: u32);
            #[cfg_attr(target_arch = "wasm32", link_name = "get_player_count: u64")]
            pub fn get_player_count() -> u64;
            #[cfg_attr(target_arch = "wasm32", link_name = "get_player(UUID): Player")]
            pub fn get_player(uuid: u64) -> *const get_player_rt;
        }
        #[repr(C)]
        pub struct get_player_rt{
            pub name_ptr: usize,
            pub name_len: u32,
            pub uuid: u64,
        }
    }

    // Functions exposed by the bindings
    pub fn get_player_count() -> u64{
        unsafe{ externs::get_player_count() }
    }
    pub fn print_log(msg: &str){
        unsafe{ externs::print_log(msg.as_ptr() as u32, msg.len() as u32); }
    }
    pub fn get_player(uuid: UUID) -> Player{
        unsafe{
            let ret = externs::get_player(uuid);
            let ret = &*ret;
            let len1 = ret.name_len as usize;
            let ret_cast = Player{
                name: String::from_utf8_unchecked( Vec::from_raw_parts(ret.name_ptr as *const u8 as *mut _, len1, len1) ),
                uuid: ret.uuid,
            };
            return ret_cast
        }
    }

    // Types
    pub type UUID = u64;
    pub struct Player{
        pub name: String,
        pub uuid: UUID,
    }
// }