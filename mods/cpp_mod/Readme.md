# C++ Mod
Only works with clang / llvm (and the lld linker).

There's a CMakeLists file (which I'm not fully happy with) to facilitate the build like how one might for a real project.

It effectively just runs:
```bash
clang++ -I"./include/" -v ^
    -std=c++26 -O3 ^
    --target=wasm32-unknown-unknown -nostdlib -fno-exceptions -fvisibility=hidden -nostartfiles -mmultivalue ^
    -Xclang -target-abi -Xclang experimental-mv ^
    -Wl,--no-entry -Wl,--strip-all ^
    -o cpp-mod.wasm main.cpp
```

TODO: Become freestanding, provide libs to link with
As it stands I'm not sure the C++ development environment actually has the tools to do this.

Current blockers:
- Freestanding libraries: `<array>`, `<tuple>` come to mind
- Getting multi-value returns to behave correctly