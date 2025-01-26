TinyGo can be downloaded from GitHub.
Requires GoLang installed on the machine.
Requires wasm-opt on the path or as an environment variable (this is actually included in the assemblyscript mod).

```bash
tinygo.exe build -size short -o ./build/go_mod.wasm -target wasm-unknown -gc=leaking -no-debug
```