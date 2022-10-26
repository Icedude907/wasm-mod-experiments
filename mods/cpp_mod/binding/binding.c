#include <stdlib.h>
#include <binding.h>

__attribute__((weak, export_name("cabi_realloc")))
void *cabi_realloc(
void *ptr,
size_t orig_size,
size_t org_align,
size_t new_size
) {
  void *ret = realloc(ptr, new_size);
  if (!ret)
  abort();
  return ret;
}

__attribute__((weak, export_name("canonical_abi_free")))
void canonical_abi_free(
void *ptr,
size_t size,
size_t align
) {
  free(ptr);
}
#include <string.h>

void binding_string_set(binding_string_t *ret, const char *s) {
  ret->ptr = (char*) s;
  ret->len = strlen(s);
}

void binding_string_dup(binding_string_t *ret, const char *s) {
  ret->len = strlen(s);
  ret->ptr = cabi_realloc(NULL, 0, 1, ret->len);
  memcpy(ret->ptr, s, ret->len);
}

void binding_string_free(binding_string_t *ret) {
  canonical_abi_free(ret->ptr, ret->len, 1);
  ret->ptr = NULL;
  ret->len = 0;
}
void binding_player_free(binding_player_t *ptr) {
  binding_string_free(&ptr->name);
}

__attribute__((aligned(8)))
static uint8_t RET_AREA[16];
__attribute__((import_module("binding"), import_name("print-chat: func(msg: string) -> unit")))
void __wasm_import_binding_print_chat(int32_t, int32_t);
void binding_print_chat(binding_string_t *msg) {
  __wasm_import_binding_print_chat((int32_t) (*msg).ptr, (int32_t) (*msg).len);
}
__attribute__((import_module("binding"), import_name("print-log: func(msg: string) -> unit")))
void __wasm_import_binding_print_log(int32_t, int32_t);
void binding_print_log(binding_string_t *msg) {
  __wasm_import_binding_print_log((int32_t) (*msg).ptr, (int32_t) (*msg).len);
}
__attribute__((import_module("binding"), import_name("get-player-count: func() -> u64")))
int64_t __wasm_import_binding_get_player_count(void);
uint64_t binding_get_player_count(void) {
  int64_t ret = __wasm_import_binding_get_player_count();
  return (uint64_t) (ret);
}
__attribute__((import_module("binding"), import_name("get-player: func(id: u64) -> record { name: string, uuid: u64 }")))
void __wasm_import_binding_get_player(int64_t, int32_t);
void binding_get_player(uint64_t id, binding_player_t *ret0) {
  int32_t ptr = (int32_t) &RET_AREA;
  __wasm_import_binding_get_player((int64_t) (id), ptr);
  *ret0 = (binding_player_t) {
    (binding_string_t) { (char*)(*((int32_t*) (ptr + 0))), (size_t)(*((int32_t*) (ptr + 4))) },
    (uint64_t) (*((int64_t*) (ptr + 8))),
  };
}
