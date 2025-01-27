#pragma once
#include <global.hpp>
// Roll your own memory allocator.
// There are promising existing projects like malloc0.
// It doesn't appear as though C++ will ever provide one by default.

// End of the stack is at the address of this. Using a funny linker trick.
extern u8 __heap_base;
inline size_t heap_base(){ return (size_t)&__heap_base; }


inline auto host_alloc(size_t pages) -> void* {
    // memory-block 0, delta = num 64k pages
    auto prev_size_pages = __builtin_wasm_memory_grow(0, pages); // clang intrinsics

    // Clang always allocates enough memory on program start to fit the stack and all other segments.
    // E.g.: This applciation uses ~0x10400 bytes => 2 pages
    // Therefore, upon allocating a third page, `prev` = 2, and we most definitely have free memory in range 0x20000..<0x30000
    auto most_definitely_free = (void*)(prev_size_pages * 65536);
    return most_definitely_free;
}