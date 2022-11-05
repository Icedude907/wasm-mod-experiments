#include <util/int.h>

#include "./bindings/demo.h"

[[clang::export_name("modmain")]] void modmain(){
    demo::counter();
    demo::print(5);
    demo::counter();
    auto res = demo::hostmath(5, 21, 3);
    auto rng = demo::rand();
    demo::print(rng);
    demo::print(res);
    demo::counter();
}