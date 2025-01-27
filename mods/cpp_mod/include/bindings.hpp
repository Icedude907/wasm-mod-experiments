#pragma onces

#include <global.hpp>
#include <malloc.hpp>
// Call me back in two years when clang has freestanding C++ headers
// #include <tuple>
// #include <array>
// #include <cstring>

namespace std{
    template<class T, const u64 N> struct array{
        T _impl[N];
    };
    template<class T, class Q> struct tuple{
        T a; Q b;
    };
    // String length excluding terminator in bytes
    usize strlen(const char* s){
        usize i = 0;
        while(*s != '\0'){ i++; s++; }
        return i;
    }
}

// #define WASM_EXPORT extern "C" // doesn't work on its own :( and I don't understand why
#define WASM_QUOTE(x) #x
#define WASM_EXPORT(name) extern "C" [[clang::export_name(WASM_QUOTE(name))]] auto name

#define WASM_IMPORT_FULL(module, name) [[using clang: import_module(module), import_name(name)]]
#define WASM_IMPORT(name) WASM_IMPORT_FULL(WASM_MODULE, name)

namespace game{
    namespace utils{
        struct ResultWithLength{
            u32 len; bool result;
            static constexpr ResultWithLength from(u64 parts){
                return ResultWithLength{.len = (u32)parts, .result = (parts >> 32) != 0 };
            }
        };
    }
    // Contains underlying functions across the abi boundary
    namespace externs{
        #define WASM_MODULE "game"
        WASM_IMPORT("hello"      ) void hello();
        WASM_IMPORT("fncounter"  ) void fncounter();
        WASM_IMPORT("printnumber") void printnumber(u32);
        WASM_IMPORT("rand64"     ) auto rand64() -> u64;
        WASM_IMPORT("recv128"    ) auto recv128() -> u128;
        WASM_IMPORT("print"      ) void print(const u8* src, u32 len);
        WASM_IMPORT("getline"    ) auto getline() -> u64; // ResultWithLength
        WASM_IMPORT("bulkdump"   ) auto bulkdump(u8* dst) -> bool;
        #undef WASM_MODULE
    }

    // API to interface with underlying functions
    auto hello(){ return externs::hello(); }
    auto fncounter(){ return externs::fncounter(); }
    auto printnumber(u32 num){ return externs::printnumber(num); }
    auto rand64(){ return externs::rand64(); }
    auto recv128(){ return externs::recv128(); }
    // TODO: std::span / std::string_view
    auto print(const char* dat){ return externs::print((u8*)dat, std::strlen(dat)); }
    auto print(u8* dat, u32 len){ return externs::print(dat, len); }
    // TODO: std::string
    const char* getline(){
        // Currently this line of text is never freed.
        // Appears as though C++ environments need to roll their own memory allocator.
        auto [len, result] = utils::ResultWithLength::from(externs::getline());

        if(!result)  return ""; // Technically nullopt
        if(len <= 0) return ""; // No allocation needed

        // Fake memory allocation (no free lol)
        auto pages = 1 + result / 65536;
        auto target_memory = (u8*)host_alloc(pages);
        externs::bulkdump(target_memory);
        target_memory[len] = '\0'; // Terminate the string

        return (char*)target_memory;
    }
}

#undef WASM_IMPORT
#undef WASM_IMPORT_FULL