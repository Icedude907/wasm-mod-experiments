#pragma once
// TODO: FIXME! Current command explodes
// #include <tuple>
// #include <array>
// #include <cstring>

#include <util/int.h>

#define WASMIMPORTFULL(module, name) [[using clang: import_module(module), import_name(name)]]
#define WASMMODULE "demo"
#define WASMIMPORT(name) WASMIMPORTFULL(WASMMODULE, name)

namespace demo{
    // Contains underlying functions across the abi boundary
    namespace externs{
        WASMIMPORT("counter()") void counter();
        WASMIMPORT("print(u32)"                      ) void print(u32);
        WASMIMPORT("rand() u64"                      ) u64  rand();
        WASMIMPORT("hostmath(u8 i16 u8) bool8"       ) u8   hostmath(u8, i16, u8);
        // WASMIMPORT("rand() (u32, bool8)"             ) std::tuple<u32, u8> rand2();
        // WASMIMPORT("print(ptr)"                      ) void print(std::array<u8, 12> &data);
        WASMIMPORT("print(ptr, u32)"                 ) void print(const char* data, u32 len);
        // WASMIMPORT("rand(ptr)"                       ) void rand(std::array<u64, 24> &outbuf);
        // WASMIMPORT("receive_big_buffer(ptr)"         ) void receive_big_buffer(std::array<u8, 104857600> &outbuf);
        WASMIMPORT("prepare_arbitrary_string() usize") u32  prepare_arbitrary_string();
        WASMIMPORT("receive_arbitrary(ptr) enum8"    ) u8   receive_arbitrary();
    }
    // API to interface with underlying functions
    inline void counter()   { externs::counter(); }
    inline void print(u32 a){ externs::print(a); }
    inline u64  rand()      { return externs::rand(); }
    inline bool hostmath(u8 a, i16 b, u8 c){
        return externs::hostmath(a,b,c) != 0;
    }
    //
    //
    // TODO:
    // inline void print(const char* string){
    //     externs::print(string, strlen(string));
    // }
    //
    //
    // inline const char* receive_arbitrary_string(){
    //     auto len = externs::prepare_arbitrary_string();
    //     // TODO:

    // }
}

#undef WASMMODULE
#undef WASMIMPORTFULL
#undef WASMIMPORT