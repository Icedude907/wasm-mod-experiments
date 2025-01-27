package main

import "fmt"

//---------
// User code
// -----------------

//go:export modmain
func modmain() {
	hello()
	fncounter()
	fncounter()
	printnumber(556677)
	printnumber(99999)
	var num = rand64();
	printnumber(uint32(num >> 32))
	printnumber(uint32(num))
	var b128 = recv128()

	print(fmt.Sprintf("The 128 bit: 0x%x_%x", b128.high, b128.low))
	var participant = getline()
	print(fmt.Sprintf("Thanks for participating, %s", participant))
}

//go:export onshutdown
func onshutdown(){
	print("All the main things seem to be working!")
}

// Mandatory stub: https://github.com/tinygo-org/tinygo/issues/2703
func main() {}
