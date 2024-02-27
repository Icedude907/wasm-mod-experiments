#include <util/int.h>

#include "./bindings/demo.h"

WASM_EXPORT(modmain)() -> void {
    demo::counter();
    demo::print(5);
    demo::counter();
    auto res = demo::hostmath(5, 21, 3);
    auto rng = demo::rand();
    demo::print(rng);
    demo::print(res);
    demo::counter();

    std::array<u8, 12> hex = {0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xfa};
    demo::print_bytes(hex);
    auto rng2 = demo::rand2();
    demo::print(rng2.a);
    demo::print(rng2.b);

    demo::print("HELLO WORLD!");
    auto buf = demo::rand_buf();
    demo::print(buf._impl[0]);

    // demo::receive_big_buffer();
}