#pragma onces

#include <util/int.h>
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

#define WASM_QUOTE_INTERNAL(x) #x
#define WASM_QUOTE(x) WASM_QUOTE_INTERNAL(x)
#define WASM_EXPORT(name) [[clang::export_name(WASM_QUOTE(name))]] auto name

#define WASMIMPORTFULL(module, name) [[using clang: import_module(module), import_name(name)]]
#define WASMMODULE "demo"
#define WASMIMPORT(name) WASMIMPORTFULL(WASMMODULE, name)

namespace demo{
    // Contains underlying functions across the abi boundary
    namespace externs{
        WASMIMPORT("counter()"                       ) void counter();
        WASMIMPORT("print(u32)"                      ) void print(u32);
        WASMIMPORT("rand() u64"                      ) u64  rand();
        WASMIMPORT("hostmath(u8 i16 u8) bool8"       ) u8   hostmath(u8, i16, u8);
        WASMIMPORT("rand() (u32, bool8)"             ) std::tuple<u32, u8> rand2();
        WASMIMPORT("print(ptr)"                      ) void print_bytes(const std::array<u8, 12>* data);
        WASMIMPORT("print(ptr, u32)"                 ) void print(const u8* data, u32 len);
        WASMIMPORT("rand(ptr)"                       ) void rand(std::array<u64, 24>* outbuf);
        WASMIMPORT("receive_big_buffer(ptr)"         ) void receive_big_buffer(std::array<u8, 104857600>* out);
        WASMIMPORT("prepare_arbitrary_string() usize") u32  prepare_arbitrary_string();
        WASMIMPORT("receive_arbitrary(ptr) enum8"    ) u8   receive_arbitrary(u8* data);
    }
    enum ReceiveArbitraryResult{
        Sucess = 0,
        GenericFailure = 1,
        NotPrepared = 2,
    };

    // API to interface with underlying functions
    inline void counter()   { externs::counter(); }
    inline void print(u32 a){ externs::print(a); }
    inline u64  rand()      { return externs::rand(); }
    inline bool hostmath(u8 a, i16 b, u8 c){
        return externs::hostmath(a,b,c) != 0;
    }
    inline std::tuple<u32, u8> rand2(){ return externs::rand2(); }
    inline void print_bytes(const std::array<u8, 12> &data){
        externs::print_bytes(&data);
    }
    inline void print(const char* data){
        externs::print(reinterpret_cast<const u8*>(data), std::strlen(data));
    }
    inline std::array<u64, 24> rand_buf(){
        u64 ret[24];
        externs::rand(reinterpret_cast<std::array<u64, 24>*>(&ret));
        return *reinterpret_cast<std::array<u64, 24>*>(&ret); // TODO: std::move()
    }
    // inline std::array<u8, 104857600> receive_big_buffer(){
    //     u8 ret[104857600];
    //     externs::receive_big_buffer(reinterpret_cast<std::array<u8, 104857600>*>(&ret));
    //     return *reinterpret_cast<std::array<u8, 104857600>*>(&ret); // TODO: std::move()
    // }
    // inline std::string readline(){
    //     auto len = externs::prepare_arbitrary_string();
    //     std::string dat(len, '\0');
    //     auto res = (ReceiveArbitraryResult)externs::receive_arbitrary(&dat[0]);
    //     if(res != ReceiveArbitraryResult::Sucess){ /*PANIC*/ }
    //     return dat;
    // }
}

#undef WASMMODULE
#undef WASMIMPORTFULL
#undef WASMIMPORT