// Bear with me I'm not a JS programmer
import * as demo from "../bindings/demo"

export function modmain(): void{
  demo.counter();
  demo.counter();
  demo.hostmath(1, demo.rand() as i16, 99);
  var buf = new StaticArray<u8>(12);
  buf = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; // MUST BE 12;
  demo.print_buffer(buf);

  demo.print("Hello from AssemblyScript!");

  let buf2 = demo.rand_buf();
  print_randbuf(buf2);

  // Yuge
  let buf3 = demo.receive_big_buffer();
  demo.print_u32(buf3[99])

  let stdin = demo.receive_string();
  demo.print("Good to meet you, " + stdin + "!");
}

export function onshutdown(): void{
  demo.print_u32(52);
}

function print_randbuf(data: Uint64Array): void{
  let s = "";
  for(let i = 0; i < data.length; i++){
    s += data[i].toString() + ", ";
  }
  demo.print(s);
}