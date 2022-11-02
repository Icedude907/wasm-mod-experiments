#![allow(non_snake_case)]
// These bindings were written by hand.
// Ideally in the future this would be fully automated, but as it stands the tools avaliable are not fit for my purposes.

/////////////////////////////////////////////////////
// "Safe" wrappings of the underlying functions.
// Unwraps, rebuilds data structs but not much else.
/////////////////////////////////////////////////////

pub mod demo{
    pub fn host_fn(){
        unsafe{ super::externs::host_fn() }
    }
    pub fn host_fn_u32(num: u32){
        unsafe{ super::externs::host_fn_u32(num) }
    }
    pub fn host_fn__u64() -> u64{
        unsafe{ super::externs::host_fn__u64() }
    }
    pub fn host_fn_u8_i16_u8__bool8(num1: u8, num2: i16, num3: u8) -> bool{
        // Have to convert back into a bool. There's very little concept of anything else otherwise
        unsafe{ super::externs::host_fn_u8_i16_u8__bool8(num1, num2, num3) != 0 }
    }
    pub fn host_fn__u32_bool8() -> (u32, bool){
        let val = unsafe{ super::externs::host_fn__u32_bool8() };
        return (val.0, val.1 != 0);
    }
    pub fn host_fn_u8x12p(buf: &[u8; 12]){
        unsafe{ super::externs::host_fn_u8x12p(buf) }
    }
    pub fn host_fn_strutf8(string: &str){
        // Convert slice to FFI-compatible struct
        let s = super::externs::slice(string.as_ptr(), string.len());
        unsafe{ super::externs::host_fn_strutf8(s) }
    }
    pub fn host_fn__u64x24p() -> [u64;24]{
        // This function is sneaky and unwraps the buffer.
        unsafe{
            let mut buf: [u64;24] = std::mem::MaybeUninit::uninit().assume_init();
            super::externs::host_fn__u64x24p(&mut buf);
            return buf;
        }
    }
    // This function handles adaptation of allocating and calling with a pointer as an argument, and passes it back as a return value.
    pub fn host_fn__hostmemtest() -> Box::<[u8;104857600]>{
        unsafe{
            let mut buf = Box::<[u8;104857600]>::new([0;104857600]); // Should use uninit (nightly)
            super::externs::host_fn__hostmemtest(buf.as_mut());
            return buf;
        }
    }
}

// Underlying unsafe functions
mod externs{
    #[link(wasm_import_module = "demo")]
    extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "host_fn()")]
        pub fn host_fn();
        #[cfg_attr(target_arch = "wasm32", link_name = "host_fn(u32)")]
        pub fn host_fn_u32(num: u32); // Notice I've had to rename the function due to rust's no-overload (sometimes) rule.
        #[cfg_attr(target_arch = "wasm32", link_name = "host_fn() u64")]
        pub fn host_fn__u64() -> u64;
        #[cfg_attr(target_arch = "wasm32", link_name = "host_fn(u8 i16 u8) bool8")]
        pub fn host_fn_u8_i16_u8__bool8(num1: u8, num2: i16, num3: u8) -> u8;
        // NOTE: If your compiler outputs a function TAKING [i32] and returning [] void, see here:
        // https://stackoverflow.com/questions/70641080/wasm-from-rust-not-returning-the-expected-types
        // See `rustflags` in ./cargo/config.toml
        #[cfg_attr(target_arch = "wasm32", link_name = "host_fn() (u32, bool8)")]
        pub fn host_fn__u32_bool8() -> a;
        #[cfg_attr(target_arch = "wasm32", link_name = "host_fn([u8;12]*)")]
        pub fn host_fn_u8x12p(buf: &[u8; 12]);
        #[cfg_attr(target_arch = "wasm32", link_name = "host_fn(strutf8)")]
        pub fn host_fn_strutf8(string: slice<u8>);
        #[cfg_attr(target_arch = "wasm32", link_name = "host_fn() [u64;24]*")]
        pub fn host_fn__u64x24p(outbuf: &mut [u64;24]);
        #[cfg_attr(target_arch = "wasm32", link_name = "host_fn() 100mb*")]
        pub fn host_fn__hostmemtest(outbuf: &mut [u8;104857600]); // Some read only memory
    }
    #[repr(C)] pub struct a(pub u32, pub u8);
    #[repr(C)] pub struct slice<T>(pub *const T, pub usize);
}