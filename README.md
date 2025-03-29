# Using Webassembly as a scripting system

> I've been thinking lately about how to use Wasmer as a general-purpose embedder for sandboxed scripting (I'm sure I'm not the only one).  
> &mdash; ElusiveMori (https://github.com/wasmerio/wasmer/issues/315)

He isn't the only one.

Many professional programs and games today have some kind of programming interface for customising their behaviour. For example:
- Factorio (game): Official Lua-based comprehensive modding api
- Minecraft Java (game): Pseudo-Unofficial Java class-mixin system
- Ghidra (tool): Integrated Python interpreter and Java runtime
- Foobar2000 (music player): Runtime DLL modding interface (Windows-MSVC x64 only)

Each of these has its own drawbacks - those being:
1. Scripting languages can be unacceptably slow for intensive operations (Ghidra Python)
2. Running bare metal is not portable (Foobar2000)
3. A non-sandboxed environment can be used to install viruses or misbehave (Minecraft Java)
    - Particularly concerning if a mod can update itself or must run on each client.
4. The learning curve for a niche language (e.g.: Lua) is often not worth the effort. The language may also be not very good. (e.g.: Lua)

Scripting in general, behaves roughly like so:
- The host sets out a set of entrypoints and fields the guest must implement to be considered valid
- The host provides a set of functions for the guest to make use of
- The host calls the guest's entrypoints at particular intervals (e.g.: once per frame), and the guest interfaces back with functions, return values, and shared memory regions to enact the desired behaviour.
- This goes on until the host orders a shutdown or the guest attempts to run illegal code (and hopefully is stopped if malicious)

WebAssembly may present us with an opportunity to address existing scripting concerns and add on some additional features:
1. It is acceptably speedy due to the existence of JIT compilation / precompilation from the likes of LLVM.
2. It can be heavily sandboxed. There are *no mandatory functions* that instruct the host's OS to make a system call.
3. It is not tied to a specific programming language. The script-writer can theoretically write in any language that implements the small wasm subset (and correct FFI behaviour, which turned out to be something of a problem)
4. It's had mainstream development work behind it. Due to it's presence in browsers there's been a lot of attention and development work put in.
5. Inter-script communication is possible with message passing and shared memory.

However its not without it's flaws. Speaking personally, there's a few:
- From the perspective of a host developer, much of the tooling is immature.
    - Documentation and guides regarding how to do things are pretty sparse
    - Bindings and documentation generation for guest languages is minimally avaliable
    - Extra effort is needed to pass structures / pack fields / send strings across the host-guest boundary 
- Some helpful features aren't supported anywhere leaving performance and usability on the table:
    - multi-value return over FFI
    - multiple-memory spaces (which puts a damper on shared memory inter-script communication for now)
- The bytecode itself has a couple of questionable decisions:
    - Typed memory (see instruction 0xBC - reinterpreting bits)
    - Functions being defined in terms of typed parameters (seems super stoobid for an assembly language)
    - Memory is linear with an inability to free (perhaps circumvented by the multi-memory proposal)
    - Web-focused proposals which would bloat the runtime (e.g.: JS garbage collection types)
- WASM development seems to be slowing down around the internet which is concerning since its not completely mature yet

Cost benefit analysis: is Webassembly-for-scripting a better option than existing tools accounting for these flaws? Maybe.  
I reckon developer usability concerns will be surmounted with time.  
Regarding language concerns, I think it will be a *long time* until a better solution is developed. WASM is unique in that it got input from all corners of the internet attempting to dethrone JS on the web, and there's probably not enough skilled parties interested in developing a secondary ecosystem tailored to modding specifically.

## The project
The project provides a runtime (written in rust with the `wasmer` library) and a collection of "mods" which can run its environment. Each mod is written in a different language and attempts to implement a minimal feature set to prove that it *could* be used as a client for a wasm modding api.

This project may be useful to you as a learning resource, but things are still changing frequently.
Early 2025's implementation has some significant differences to 2023's for example.

**Perpetually unfinished, stuff's pretty messy, docs are only partial, buildscripts are incomplete.**.

Read more about the intricacies of each implementation in each mod folder.
