// Primitive data types
// int, float, bool, char

// Integer
// Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
// i8, i16, i32, i64, i128: Signed integers.
// u8, u16, u32, u64, u128: Signed integers.

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

// diff bw i32 (32 bits) and i63(64 bita)
// range :
// i32 - 214783647
// i64 - 9223372036854775807
let e: i32 = 214783647;
let i: i64 = 9223372036854775807;
println!("Maximum value of i32: {}", e);
println!("Maximum value of i64: {}", i);
// =====================================

// Floats[Floating Point Types]
// Floats
// f32, f64
let pi: f64 = 3.14;
println!("Value of Pi: {}", pi);

// Boolean Values: true of false
let is_snowing: bool = true;
println!("Is it snowing? {}", is_snowing);

    // Character Type - char
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}