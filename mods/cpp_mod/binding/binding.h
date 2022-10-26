#ifndef __BINDINGS_BINDING_H
#define __BINDINGS_BINDING_H
#ifdef __cplusplus
extern "C"
{
  #endif
  
  #include <stdint.h>
  #include <stdbool.h>
  
  typedef struct {
    char *ptr;
    size_t len;
  } binding_string_t;
  
  void binding_string_set(binding_string_t *ret, const char *s);
  void binding_string_dup(binding_string_t *ret, const char *s);
  void binding_string_free(binding_string_t *ret);
  typedef struct {
    binding_string_t name;
    uint64_t uuid;
  } binding_player_t;
  void binding_player_free(binding_player_t *ptr);
  void binding_print_chat(binding_string_t *msg);
  void binding_print_log(binding_string_t *msg);
  uint64_t binding_get_player_count(void);
  void binding_get_player(uint64_t id, binding_player_t *ret0);
  #ifdef __cplusplus
}
#endif
#endif
