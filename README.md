# Using Webassembly as a scripting system

> I've been thinking lately about how to use Wasmer as a general-purpose embedder for sandboxed scripting (I'm sure I'm not the only one).  
> &mdash; ElusiveMori (https://github.com/wasmerio/wasmer/issues/315)

He isn't.

Many professional programs and games today have some kind of programming interface for customising their behaviour. For example:
- Factorio (game): Official Lua-based comprehensive modding api
- Minecraft Java (game): Pseudo-Unofficial Java class-mixin system
- Ghidra (tool): Integrated Python interpreter and Java runtime
- Foobar2000 (music player): Runtime DLL modding interface (Windows-MSVC x64 only)

Each of these has its own drawbacks - those being:
1. Scripting languages can be unacceptably slow for intensive operations (Factorio)
2. Running bare metal is not portable (Foobar2000)
3. A non-sandboxed environment can be used to install viruses or just misbehave (Minecraft)
    - Particularly of concern when the code must be run on each client, or mods have an automatic updater.
4. The learning curve for a niche scripting language (e.g.: Lua) is often not worth the effort. The language may also be not very good. (e.g.: Lua)

Scripting in general, behaves roughly like so:
- The host sets out a set of entrypoints and fields the guest must implement to be considered valid
- The host provides a set of functions for the guest to make use of
- The host calls the guests entrypoints at particular intervals (e.g.: once per frame), and the guest interfaces back with functions, return values, and shared memory regions to enact the desired behaviour.
- This goes on until the host orders a shutdown or the guest attempts to run illegal code (and hopefully doesn't discover a vulnerability and kill everything)

WebAssembly may present us with an opportunity to address existing scripting concerns and add on some additional features:
1. It is acceptably speedy due to the existence of JIT compilation / precompilation from the likes of LLVM.
2. It can be heavily sandboxed. There are *no mandatory functions* that instruct the host's OS to make a system call.
3. It is not tied to a specific programming language. The script-writer can theoretically write in any language that implements the small wasm subset (and correct FFI behaviour, which turned out to be something of a problem)
4. It's had mainstream development work behind it. Due to it's presence in browsers there's been a lot of attention and development work put in.

However its not without it's flaws. There's a few:
- From the perspective of a developer for the host, much of the tooling is immature.
    - Documentation and guides regarding how to do things are pretty sparse
    - Generating documentation for end users
    - Generating bindings for targets
    - Some extra effort is needed to pass structures / pack fields / send strings across the boundary (low level memory manipulation of the target wasm instance's memory space)
- Some helpful features aren't supported anywhere leaving performance and usability on the table:
    - multi-value return over FFI
    - multiple-memory spaces
- There's a couple of language design flaws:
    - Typed memory
    - Typed parameters on functions (seems super stoobid)
    - Linear memory with inability to free (could be circumvented with multi-memory but this would require a new pointer type in most languages)
        - Wasm <-> Wasm IPC is hindered by this.
    - Web-focused proposals which could bloat the runtime (e.g.: JS garbage collection types)
- And commenting as an observer, development appears to be slowing down and maturing, so many of these things will not be broadly fixed for a long time I wager.

Cost benefit analysis: is Webassembly-for-scripting a better option than existing tools even accounting for these flaws?
Many of the usability concerns could be surmounted with time I reckon.  
In regards to the language design flaws: it will be a *long time* until internet nerds get around to developing a better solution. The desperation to destroy JS resulted in WASM getting input from all corners of the internet, and developing a secondary ecosystem specifically tailored for scripting probably doesn't have enough skilled interested parties to make it happen.

## The project
The project provides a runtime (written in rust with a library called `wasmer`) and a collection of "mods" which can run in the runtime's environment. Each mod is written in a different language and attempts to implement a minimal feature set to prove that it *could* be used as a client for a wasm modding api.

This project may be useful to you as a learning resource, but things are changing relatively quickly.
Early 2025's implementation has some significant differences to 2023's for example.

**Perpetually unfinished, stuff's pretty messy, docs are only partial, buildscripts are incomplete.**.

Read more about the intricacies of each implementation in each mod folder.
