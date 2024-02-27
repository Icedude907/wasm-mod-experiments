#pragma once

#define WASM_QUOTE_INTERNAL(x) #x
#define WASM_QUOTE(x) WASM_QUOTE_INTERNAL(x)

#define WASM_EXPORT(name) [[clang::export_name(WASM_QUOTE(name))]] auto name