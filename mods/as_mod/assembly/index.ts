import {} from "../bindings/env" // Just for syntax highlighting

import * as game from "../bindings/game"

export function modmain(): void{
    game.print("Hello from AssemblyScript!");
    game.printnumber(<u32>(Math.random() * 10000))
    let line = game.getline();
    if(line != null){
        game.print(`Hello, ${line}`);
    }else{
        game.print("Got null :(");
    }
    let r128 = game.recv128();
    game.print(`recv128 test: 0x${r128.high.toString(16)}_${r128.low.toString(16)}`)
}

export function onshutdown(): void{
    game.print("Ok bye")
}

// function print_randbuf(data: Uint64Array): void{
//   let s = "";
//   for(let i = 0; i < data.length; i++){
//     s += data[i].toString() + ", ";
//   }
//   demo.print(s);
// }