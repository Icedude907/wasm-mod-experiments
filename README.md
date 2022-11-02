# WASM as a mod / scripting system

> I've been thinking lately about how to use Wasmer as a general-purpose embedder for sandboxed scripting (I'm sure I'm not the only one).  
> &mdash; ElusiveMori (https://github.com/wasmerio/wasmer/issues/315)

Many games & tools today have scripting languages they use for customising their functionality (i.e.: mods).

Essentially:
- The host exposes a bunch of functions to the script (this is the main program).
- The guest calls a bunch of functions from the script (this is the mod), and exposes a few functions of its own (callbacks, mostly).
- The host pings the guest's exposed functions as it needs to.
- This two way communication carries on as long as need be until the host destroys the guest (or the guest finds a vulnerability and destroys everything).

WASM presents a unique opportunity for this:
- It supports this mechanism of function calling and function exposure, like a dynamic library.
- Its faster than most other non-native code by virtue of being precompiled and operating a small instruction set.
- It's becoming mainstream. Many languages can already compile to WASM (with some caveats). This is both great for the program developers who can get setup quickly, and good for modder's who can pick a language of their choosing and expect reasonable performance.
- Almost all bloat is optional. Though it is made for the "web", it certainly doesn't need to run on it.

The following is a "runtime" (in rust) and a collection of "mods" (many languages) that can run in the runtime's environment.  
This project may be useful to you as a learning resource, but things are changing quickly, and I expect many systems here to be replaced.  
The project is intended to be a minimum viable example, with documentation as it goes.
**Currently unfinished, stuff's pretty messy, docs are only partial, & pointers are broken**.


### Project operation
- `runtime` contains the host virtual machine. It operates a *"game"* where *"players"* can join. It exposes a few functions to the WASM modules which can query some details and write some output.
- `mods` contains different implementations of this interface in different languages. They behave slightly different to eachother (some don't define optional functions, others only make use of a subset of host functions)

Check the releases tab for precompiled mods + the runtime (x64 Windows as of right now).

-------
### Building:
- `runtime/`: `cargo build`  
    Takes a while due to building the dependencies
- `mods/rust_mod/`: `cargo build --release`  
    Requires the toolchain `wasm32-unknown-unknown`.
    (If you are completely lost, start [here (rust-lang.org)](https://www.rust-lang.org/learn/get-started)).
    Output is in `./target/wasm32(...)/release/*.wasm`
- `mods/cpp_mod`: TODO  
    *(Likely requires `clang` & `cmake` + a build system.)*

-------
## Personal thoughts
The toughest parts of all of this were:
- Figuring out how `wasmer`'s memory modification system worked on the host's end to parse array pointers and strings (poor docs / seemingly unnecessary complications).
- Crafting bindings by hand. Tools are starting to be made, but they are far from complete and aren't usable yet.

Also, some minor annoyances were had in setting WASM mods up to strip down fluff (e.g.: symbols) into a small binary.
I got it working, but future developer's just learning the system are likely to miss such things without using a template.

In closing, I'd say WebAssembly is **not** ready to do something like this **yet**.  
It is definitely possible, but the work is non-trivial.
As standards continue to get developed and implemented, this will become less so.  
The future seems bright for WASM in this use case.

## Projects with promise:
These tools aren't being used by this project in any capacity, but may prove useful for future developers. _No guarantees_
- WebAssembly Interface Types: https://github.com/WebAssembly/interface-types/blob/main/proposals/interface-types/Explainer.md
- `wit-bindgen` turns a platform-agnostic bindings definition into a drop in file to map functions between host and guest (unfinished). Uses the "canonical" ABI.

## Software I used referencing:
- https://github.com/feather-rs/feather