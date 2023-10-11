pub fn matchh() {
    match_needles();
}

fn match_needles() {
    let haystack = [1, 1, 2, 5, 14, 42, 132, 420, 429, 1430, 4938];

    for item in &haystack {
        let result = match item {
            42 | 420 => "hit",
            _ => "miss", // _ wildcard. match everything else
        };

        if result == "hit" {
            println!("{}: {}", item, result)
        }
    }
}
