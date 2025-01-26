// Intended to be used as wasm32-unknown-unknown module. To compile it, run:
// tinygo build -size short -o hello-unknown.wasm -target wasm-unknown -gc=leaking -no-debug
package main

//go:wasmimport demo print(u32)
func print(x uint32)

//go:export modmain
func modmain() {
	print(101)
}

// Still mandatory: https://github.com/tinygo-org/tinygo/issues/2703
func main() {}
