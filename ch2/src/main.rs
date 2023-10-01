fn adding_ints() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("a + b + c + d = {}", e);
    // ^ macro (!) -- returns code rather than values ...
    /*
    - when printing to the console, every input data type has its own way of
    being represented as a text string.
    - `println!()` figures out what methods to call on the arguments.
    */
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn main() {
    adding_ints();
}
