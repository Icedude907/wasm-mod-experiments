#include <global.hpp>
#include <bindings.hpp>
#include <malloc.hpp>

WASM_EXPORT(modmain)() -> void {
    game::hello();
    game::fncounter();
    game::fncounter();
    game::printnumber(heap_base());
    auto num = game::rand64();
    game::printnumber(num);
    auto b128 = game::recv128();
    game::print((u8*)&b128, sizeof(b128));
    game::print("Echo test. Please give me a message");
    auto str = game::getline();
    game::print(str);
}

WASM_EXPORT(onshutdown)() -> void {
    game::print("ok bye");
}