pub fn references() {
    example_one();
    needle_haystack();
    needle_haystack_2();
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

fn needle_haystack() {
    let needle = 0o204; // 132
    let haystack = [1, 1, 2, 5, 14, 42, 132, 420, 429, 1430, 4938];

    for item in &haystack {
        // ^ iterates over references to elements within haystack
        if *item == needle {
            // ^ the syntax `*item` returns the item's referent
            println!("{}", item)
        }
    }
}

// this is the example that's currently in the book's GH repo (first example is
// from the book itself)
fn needle_haystack_2() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 420, 429, 1430, 4938];

    for reference in haystack.iter() {
        let item = *reference;
        if item == needle {
            println!("{}", item);
        }

        if reference == &needle {
            println!("{}", reference);
        }

        // (this part is mine ... :dropfox:)
        if item == &needle * 10 {
            println!("{}", item);
        }
    }
}
