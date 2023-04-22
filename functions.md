## Runtime Exposed Functions:
(By function name, not parameter layout)

NOTE: The binding's api for individual languages may change these signatures to be more idiomatic.
This document's primary concern is passing data across the WASM / language boundary.

### Namespace `demo`
- `counter()                ` => Prints some text. Keeps track of calling count.
- `print(u32)               ` => Prints a u32,
- `rand() u64               ` => Returns a random u64. Use for seeding, etc.
- `hostmath(u8 i16 u8) bool8` => Runs the math `(num1+num2)%num3 == 0` and prints the output on the host
- `rand() (u32, bool8)      ` => Returns two randoms.
- `print(ptr)               ` => Prints a 12 byte buffer given a pointer to the data `ptr = [u8;12]*`
- `print(ptr, u32)          ` => Prints a utf8 string. `strutf8{data: [charutf8;_]*, bytelen: u32}`
- `rand(ptr)                ` => Fills an array of u64s with random numbers. Requires a pointer to an allocated buffer: `ptr = [u64;24]*`
- `receive_big_buffer(ptr)  ` => Fills an ~100mb (104857600 elements) array of u8s with randoms. `ptr = [u8;104857600]*`
- `prepare_arbitrary_string() usize` => Prepares the host to send arbitrary length data (in this case a string). Returns the length of the arbitrary data to be sent to the guest.
- `receive_arbitrary(ptr) enum8` => Writes arbitrary data to the provided pointer. Returns an enum indicating the status of the transferal.
    + Expected to follow a `prepare_` function. After calling this function (regardless of result), the host drops the data. Subsequent calls will return `NOT_PREPARED` until another `prepare_` function is called.
    + `enum result{ SUCCESS = 0, GENERIC_FAIL = 1, NOT_PREPARED = 2}`
    + Many bindings will offer a utility function to bundle arbitrary length data behaviour.
    + This function is used for all arbitrary-length data transferal functions (only prepare_arbitrary_string atm)

## Mod Exposed Functions
(No namespace)
- `modmain` => Called on mod initialisation.
- `onshutdown` => Optional. Called just before a mod should exit.