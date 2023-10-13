pub fn references() {
    example_one();
}

fn example_one() {
    // imagine a is a large array (costly to duplicate). by creating a reference
    // to a, a's address is stored in memory, instead of creating a duplicate copy.
    // when the data from a is required, r can be dereferenced to make a available.
    let a = 12;
    let r = &a; // r is a reference to a

    // looking forward to understanding why my scratch code is
    // compiling and running without issue
    // let s = r + 2;
    // println!("{}", s);
    // let c = a + 2;
    // println!("{}", c);
    //

    let b = a + *r; // dereferencing r
    println!("a + a = {}", b);
}
