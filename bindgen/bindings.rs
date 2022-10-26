#[allow(clippy::all)]
mod guest_import {
  #[derive(Clone)]
  pub struct Player {
    pub name: String,
    pub uuid: Uuid,
  }
  impl core::fmt::Debug for Player {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Player").field("name", &self.name).field("uuid", &self.uuid).finish()}
  }
  pub type Uuid = u64;
  /// Prints a message to the chat
  pub fn print_chat(msg: & str,) -> (){
    unsafe {
      let vec0 = msg;
      let ptr0 = vec0.as_ptr() as i32;
      let len0 = vec0.len() as i32;
      #[link(wasm_import_module = "guest_import")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "print-chat: func(msg: string) -> unit")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "guest_import_print-chat: func(msg: string) -> unit")]
        fn wit_import(_: i32, _: i32, );
      }
      wit_import(ptr0, len0);
      ()
    }
  }
  /// Prints a message to the log
  pub fn print_log(msg: & str,) -> (){
    unsafe {
      let vec0 = msg;
      let ptr0 = vec0.as_ptr() as i32;
      let len0 = vec0.len() as i32;
      #[link(wasm_import_module = "guest_import")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "print-log: func(msg: string) -> unit")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "guest_import_print-log: func(msg: string) -> unit")]
        fn wit_import(_: i32, _: i32, );
      }
      wit_import(ptr0, len0);
      ()
    }
  }
  /// Gets the number of players currently online
  pub fn get_player_count() -> Uuid{
    unsafe {
      #[link(wasm_import_module = "guest_import")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "get-player-count: func() -> u64")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "guest_import_get-player-count: func() -> u64")]
        fn wit_import() -> i64;
      }
      let ret = wit_import();
      ret as u64
    }
  }
  /// Gets a player's data given an id.
  pub fn get_player(id: u64,) -> Player{
    unsafe {
      let ptr0 = __GUEST_IMPORT_RET_AREA.0.as_mut_ptr() as i32;
      #[link(wasm_import_module = "guest_import")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "get-player: func(id: u64) -> record { name: string, uuid: u64 }")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "guest_import_get-player: func(id: u64) -> record { name: string, uuid: u64 }")]
        fn wit_import(_: i64, _: i32, );
      }
      wit_import(wit_bindgen_guest_rust::rt::as_i64(id), ptr0);
      let len1 = *((ptr0 + 4) as *const i32) as usize;
      Player{name:String::from_utf8(Vec::from_raw_parts(*((ptr0 + 0) as *const i32) as *mut _, len1, len1)).unwrap(), uuid:*((ptr0 + 8) as *const i64) as u64, }
    }
  }
  
  #[repr(align(8))]
  struct __GuestImportRetArea([u8; 16]);
  static mut __GUEST_IMPORT_RET_AREA: __GuestImportRetArea = __GuestImportRetArea([0; 16]);
}
