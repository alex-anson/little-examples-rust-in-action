mod beware_floats;
mod loops;
mod try_into;

use crate::beware_floats::beware_floats;
use crate::loops::loops;
use crate::try_into::using_try_into;

fn adding_ints() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("a + b + c + d = {}", e);
    // ^ formatting macro (!) -- returns code rather than values ...
    /*
    - when printing to the console, every input data type has its own way of
    being represented as a text string.
    - `println!()` figures out what methods to call on the arguments.
    */
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

// SECTION: NUMBERS
// CREATING AND PERFORMING OPERATIONS ON NUMERIC TYPES

// Numeric literals and basic operations on numbers in Rust. Listing 2.3
fn numeric_types() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22_i32; // type suffix
    let add = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, add);

    let one_million: i64 = 1_000_000; // underscores = readability
    println!("{}", one_million.pow(2)); // numbers have methods. i.e., not doing
                                        // something like pow(one_million, 2)

    let forty_twos = [42.0, 42f32, 42.0_f32]; // array. all elements must be of
                                              // the same type (here, float 32)
    println!("{:012}", forty_twos[0]); // 000000000042 (total of 12 characters)
}

// Using base 2, base 8, and base 16 numeric literals
fn base_notation() {
    let three = 0b11; // 0b prefix indicates binary numerals
    let thirty = 0o36; // 0o = octal
    let three_hundred = 0x12c; // 0x = hexidecimal

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}

// Type casting
fn number_comparison() {
    let a = 10_i32;
    let b = 100_u16;
    // can't compare different types of numbers. must type cast
    if a < (b as i32) {
        println!("10 is less than 100");
    }
    // safest to cast the smaller type to the larger one (called "promotion")

    let a = 45_i8;
    let b = 300_i32;
    if a < (b as i8) {
        println!("45 is less than 300");
    } else {
        println!("DON'T TYPE CAST CARELESSLY: {}", b as i8)
    }
}

fn main() {
    adding_ints();
    println!("{:1$}", "", 200); // essentially prints two (blank) lines
    numeric_types();
    println!("{:1$}", "", 200);
    base_notation();
    println!("{:1$}", "", 200);
    number_comparison();
    println!("{:1$}", "", 200);
    using_try_into();
    println!("{:1$}", "", 200);
    beware_floats();
    println!("{:1$}", "", 200);
    loops();
}
