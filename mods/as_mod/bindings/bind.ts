
// Overriding the special functions AssemblyScript may require (your runtime environment should support at least these)


export function abort(message: usize, fileName: usize, line: u32, column: u32): void {
    throw new Error("aborted");
}

/// https://www.assemblyscript.org/concepts.html#special-imports=

// export function trace(message: usize, n: i32, a0..4?: f64): void{
// }

// // TODO:
// export function seed(): f64{
//     return 0.0
// }