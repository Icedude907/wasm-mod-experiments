package main

import "unsafe"

// ------------
// Direct bindings
// ------------------

//go:wasmimport game hello
func hello()
//go:wasmimport game fncounter
func fncounter()
//go:wasmimport game printnumber
func printnumber(uint32)
//go:wasmimport game rand64
func rand64() uint64
//go:wasmimport game recv128
func __impl_recv128(dst uintptr)
func recv128() uint128_split {
	var result uint128_split;
	__impl_recv128(uintptr(unsafe.Pointer(&result)))
	return result
}
//go:wasmimport game print
func _impl_print(src uintptr, len uint32)
func print(dat string){
	ptr, len := stringToPtr(dat)
	_impl_print(ptr, len)
}
//go:wasmimport game getline
func _impl_getline() uint64
//go:wasmimport game bulkdump
func _impl_bulkdump(dst uintptr) uint32

func getline() string {
	// Call function
	var result = unpackResultWithLength(_impl_getline())
	if(!result.ok){ return "" }
	// Allocate memory to receive
	var recv = make([]byte, result.len)
	var _ = _impl_bulkdump(uintptr(unsafe.Pointer(&recv[0]))) != 0
	// if(!ok){ return "" } // Unfallible
	return string(recv[:]) // I think this is making an extra copy
}


// -------------
// Utilities
// --------------

// https://github.com/tinygo-org/tinygo/issues/3010
// stringToPtr returns a pointer and size pair for the given string in a way
// compatible with WebAssembly numeric types.
func stringToPtr(s string) (uintptr, uint32) {
	// The pointer to the characters is extracted by converting to a byte array and taking a reference to the first byte.
	// Supposedly this copy is optimised out. https://go101.org/q-and-a/take-string-byte-addresses.html
	buf := []byte(s)
	unsafePtr := uintptr(unsafe.Pointer(&buf[0]))
	return unsafePtr, uint32(len(buf))
}
// Go doesnt support 128-bit integers naturally
// but this struct gives the correct representation for a 128 bit value sent over the wire
type uint128_split struct {
	low uint64
	high uint64
}
func unpackResultWithLength(packed uint64) struct{ok bool; len uint32} {
	return struct{ok bool; len uint32}{ ok: uint32(packed >> 32) != 0, len: uint32(packed) }
}