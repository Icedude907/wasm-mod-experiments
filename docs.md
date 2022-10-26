# Sample mod API:
Unfortunately I don't think the webassembly text format is very nice to read so these signatures are C-styled.

## WASM exports
### Required
```C++
void modmain();
void shutdown();
```
### Optional
```C++
void on_player_join(UUID id);
```

## WASM exposed functions
```C++
void print_chat(str msg);
void print_log(str msg);
u64 get_player_count();
Player get_player(UUID id);
```

## ABI
```C++
struct str{
    char* data;
    u32 len;
};
struct Player{
    str data;
    UUID id;
};
using UUID = u64;
```