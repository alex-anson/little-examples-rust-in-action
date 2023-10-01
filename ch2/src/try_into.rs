use std::convert::TryInto; // the `use` keyword brings the std::convert::TryInto
                           // TRAIT into the local scope.
                           // the code runs without importing this tho
pub fn using_try_into() {
    let a = 10_i32;
    let b = 100_u16;

    let b_ = b.try_into().expect("a u16 should fit into an i32");
    // (book uses unwrap instead of expect)

    // type of b_ is i32. Rust is looking at where b_ is used to make that
    // determination (try changing a's type to i64)

    if a < b_ {
        println!("10 is still less than 100");
    }
}
