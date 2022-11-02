#![allow(non_snake_case)]
// These bindings were written by hand.
// Ideally in the future this would be fully automated, but as it stands the tools avaliable are not fit for my purposes.

/////////////////////////////////////////////////////
// "Safe" wrappings of the underlying functions.
// Unwraps, rebuilds data structs but not much else.
/////////////////////////////////////////////////////

pub mod demo{
    pub fn counter(){
        unsafe{ super::externs::counter() }
    }
    pub fn print_u32(num: u32){
        unsafe{ super::externs::print_u32(num) }
    }
    pub fn rand() -> u64{
        unsafe{ super::externs::rand() }
    }
    pub fn hostmath(num1: u8, num2: i16, num3: u8) -> bool{
        // Have to convert back into a bool. There's very little concept of anything else otherwise
        unsafe{ super::externs::hostmath(num1, num2, num3) != 0 }
    }
    pub fn rand_2() -> (u32, bool){
        let val = unsafe{ super::externs::rand_2() };
        return (val.0, val.1 != 0);
    }
    pub fn print_buffer(buf: &[u8; 12]){
        unsafe{ super::externs::print_buffer(buf) }
    }
    pub fn print(string: &str){
        // Convert slice to FFI-compatible struct
        let s = super::externs::slice(string.as_ptr(), string.len());
        unsafe{ super::externs::print(s) }
    }
    pub fn rand_buffer() -> [u64;24]{
        // This function is sneaky and unwraps the buffer.
        unsafe{
            let mut buf: [u64;24] = std::mem::MaybeUninit::uninit().assume_init();
            super::externs::rand_buffer(&mut buf);
            return buf;
        }
    }
    // This function handles adaptation of allocating and calling with a pointer as an argument, and passes it back as a return value.
    pub fn receive_big_buffer() -> Box::<[u8;104857600]>{
        unsafe{
            let mut buf = Box::<[u8;104857600]>::new([0;104857600]); // Should use new_uninit (nightly)
            super::externs::receive_big_buffer(buf.as_mut());
            return buf;
        }
    }
}

// Underlying unsafe functions
mod externs{
    #[link(wasm_import_module = "demo")]
    extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "counter()")]
        pub fn counter();

        #[cfg_attr(target_arch = "wasm32", link_name = "print(u32)")]
        pub fn print_u32(num: u32);

        #[cfg_attr(target_arch = "wasm32", link_name = "rand() u64")]
        pub fn rand() -> u64;

        #[cfg_attr(target_arch = "wasm32", link_name = "hostmath(u8 i16 u8) bool8")]
        pub fn hostmath(num1: u8, num2: i16, num3: u8) -> u8;

        // NOTE: If your compiler outputs a function TAKING [i32] and returning void, see here:
        // https://stackoverflow.com/questions/70641080/wasm-from-rust-not-returning-the-expected-types
        // See `rustflags` in ./cargo/config.toml
        #[cfg_attr(target_arch = "wasm32", link_name = "rand() (u32, bool8)")]
        pub fn rand_2() -> a; // Notice I've had to rename the function due to rust's no-overload (sometimes) rule.

        #[cfg_attr(target_arch = "wasm32", link_name = "print(ptr)")]
        pub fn print_buffer(buf: &[u8; 12]);

        #[cfg_attr(target_arch = "wasm32", link_name = "print(ptr, u32)")]
        pub fn print(string: slice<u8>);

        #[cfg_attr(target_arch = "wasm32", link_name = "rand(ptr)")]
        pub fn rand_buffer(outbuf: &mut [u64;24]);

        #[cfg_attr(target_arch = "wasm32", link_name = "receive_big_buffer(ptr)")]
        pub fn receive_big_buffer(outbuf: &mut [u8;104857600]); // Some read only memory
    }
    #[repr(C)] pub struct a(pub u32, pub u8);
    #[repr(C)] pub struct slice<T>(pub *const T, pub usize);
}

// trait Exports{
//     #[export_name = "modmain"]
//     fn modmain()
// }