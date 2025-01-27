// Overriding the special functions AssemblyScript may require (your runtime environment should support at least these)
// https://www.assemblyscript.org/concepts.html#special-imports
import * as game from "./game"

function abort(message: usize, fileName: usize, line: u32, column: u32): void {
    game.print("A critical error occurred.")
    throw new Error("aborted");
}

// function trace(message: usize, n: i32, a0..4?: f64): void{ }

function seed(): f64 {
    return reinterpret<f64>(game.rand64())
}