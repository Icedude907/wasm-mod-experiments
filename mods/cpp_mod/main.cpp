#include <stdint.h>

#include "./binding/binding.h"

// Ok these bindings are definitely for C. No C++ autocasting or namespacing nice things here.
void modmain(){
    binding_string_t b; binding_string_set(&b, "Hello from the mod!");
    binding_print_chat(&b);
}