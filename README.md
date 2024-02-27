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
- Almost all bloat is optional. Though it is made for the "web", it certainly doesn't need to run on it. NOTE: However, that some of the more recent proposals may make this more annoying.

The following is a "runtime" (in rust) and a collection of "mods" (many languages) that can run in the runtime's environment.  
This project may be useful to you as a learning resource, but things are changing quickly, and I expect many systems here to be replaced.  
The project is intended to be a minimum viable example, with documentation as it goes.

**Currently unfinished, stuff's pretty messy, docs are only partial, & pointers are broken**.


### Project operation
- `runtime` contains the host virtual machine. It operates a *"game"* where *"players"* can join. It exposes a few functions to the WASM modules which can query some details and write some output.
- `mods` contains different implementations of this interface in different languages. They behave slightly different to eachother (some don't define optional functions, others only make use of a subset of host functions)
    - `src/bindings` contains function wrappers to provide an interface between this mod and the runtime. These would be distributed by the runtime developer.
Check the releases tab for precompiled mods + the runtime (x64 Windows as of right now).

-------
### Building:
- `runtime/`: `cargo build`  
    Takes a while due to building the dependencies
- `mods/rust_mod/`: `cargo build --release`  
    Requires the toolchain `wasm32-unknown-unknown`.
    (If you are completely lost, start [here (rust-lang.org)](https://www.rust-lang.org/learn/get-started)).
    Output is in `./target/wasm32(...)/release/*.wasm`
- `mods/cpp_mod`: TODO: In progress
- `mods/as_mod`: `npm run asbuild`
    Requires node.js, builds `./build/release.wasm`

-------
## Personal thoughts
The rust host was an interesting project, not without some issues:
- Figuring out how `wasmer`'s memory modification system worked on the host's end to parse array pointers and strings (poor docs / seemingly unnecessary complications).

Each guest presented its own set of issues, too.

Rust:
- Probably the most convenient of the backends
- Minor annoyance stripping fluff from the binary
- Crafting bindings by hand

CPP/Clang:
- Still incomplete
- Using some options multi-value is enabled, but the standard library cannot be used freestanding, so much is still tough to implement.
- Crafting bindings by hand

AssemblyScript:
- Lacks Multi-Value support disqualifying it from accessing a number of functions (this is being worked on?)
- Crafting bindings by hand

In closing, I'd say WebAssembly is **not** ready to do something like this **yet**.  
It is definitely possible, but the work is non-trivial.
As standards continue to get developed and implemented, this will become less so.  
The future seems bright-ish for WASM in this use case.

## WASM Issues
NOTE: WASM has a few core issues, one being neatly demonstrated by the Multi-Value proposal.
Essentially, on the binary level, WASM FFI functions take parameters `e.g.: (i32, f64, i64) -> (i32, i32)`.
(This is stoobid. They should just use have taken in a chunk of bits in and out, and let the bindings assign real meaning to the data `e.g.: b160 -> b64`)

Anyway, up until recently you could only return 1 'type' from functions meaning longer types needed to:
1. Reserve WASM memory for the return value
2. Pass a pointer to that space to the host as a parameter
3. The host writes the return data there
(Note that this is similar to the method of passing arbitrary-length data.)

Anyway, not everyone supports the multi-value return method yet (AssemblyScript) which severely hampers usability or speed.
In some mods I've commented out functions using that method of returning as a temporary measure - hopefully this is not the case in the future.

Though, so long as one guest language compiles mod developers can get things done, it may just be slightly annoying.

Additionally, there are some interesting language proposals on the verge of implementation that may make hosts more heavy-duty for no benefit to this application (e.g.: garbage collection)
This may make things more annoying in the future.
Though, do pay attention to Memory64 and reference-types

----
## Projects with promise:
These tools aren't being used by this project in any capacity, but may prove useful for future developers.
- `wit-bindgen` turns a platform-agnostic bindings definition into a drop in file to map functions between host and guest (unfinished). Uses the "canonical" ABI.

## Software I referenced:
Numerous, of particular note:
- https://github.com/feather-rs/feather