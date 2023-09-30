use std::thread;

fn greet_world() {
    println!("HELLO WORLD 🌎");
    // `!` signals use of a macro. think of macros as fancy functions for now.
    // offer the ability to avoid boilerplate code.

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";

    let regions = [southern_germany, japan];

    for region in regions.iter() {
        println!("{}", &region); // borrow region for read-only access
    }
}

fn penguins() {
    let penguin_data = "\
        common name,length (cm)
        Little penguin,33
        Yellow-eyed penguin  ,   65
        Fiordland penguin,60
        Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, rec) in records.enumerate() {
        if i == 0 || rec.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = rec.split(",").map(|field| field.trim()).collect();

        // will not be in release builds: if cfg!(debug_assertions) { ... }
        // create release build: cargo run --release
        if cfg!(debug_assertions) {
            println!();
            eprintln!("debug: {:?} -> {:?}", rec, fields)
        }

        // let name = fields[0];
        // if let Ok(length) = fields[1].parse::<f32>() {
        //     println!("{}, {}cm", name, length);
        // } else if let Err(error_message) = fields[1].parse::<f32>() {
        //     eprintln!("refactor #1 ... {}", error_message);
        // }

        // if let Err(fox) = fields[1].parse::<f32>() {
        //     eprintln!("the error message ... {}", fox)
        // }

        // REFACTOR #2
        let result = fields[1].parse::<f32>();

        match result {
            Ok(length) => println!("{}, {}cm", fields[0], length),
            Err(msg) => eprintln!("refactor #2 ✨  {}", msg),
        }
    }
}

// EXAMPLE: NO DANGLING POINTERS
// #[derive(Debug)] // original line
#[derive(Debug, Clone, Copy)]
enum Cereal {
    // Barley,           commented out cuz i'm not using them
    // Millet,
    // Rice,
    Rye,
    // Spelt,
    Wheat,
}

fn cereal() {
    // let mut grains: Vec<Cereal> = vec![]; // original line
    let mut grains: Vec<Cereal> = vec![Cereal::Wheat];
    grains.push(Cereal::Rye);
    // drop(grains); // original line
    drop(grains.clone());

    println!("{:?}", grains);
}

// EXAMPLE: PREVENTING RACE CONDITION
fn race() {
    let mut data = 100;

    // let handler = thread::spawn(move || {
    thread::spawn(move || {
        data = 500;
        println!("{}", data) // prints 500
    });
    let handler2 = thread::spawn(move || {
        data = 1000;
        println!("{}", data)
    });

    // handler.join().unwrap();  <- using this made the print lines switch order
    handler2.join().unwrap(); // without this, 1000 doesn't get printed

    // with the other fns commented out (for noise reduction) -> prints 500, 1000, then 100.
    // now prints 1000, 500, 100
    // this must be what the book means about not knowing which thread will run first

    println!("{}", data) // prints 100
}

// EXAMPLE: ITERATOR INVALIDATION
// fn letters() {
//     let mut letters = vec!["a", "b", "c"];

//     for letter in letters {
//         println!("{}", letter);
//         letters.push(letter.clone());
//     }
// }
fn letters_mine() {
    let letters = vec!["a", "b", "c"];
    let mut new_letters = letters.clone();

    for letter in letters {
        println!("{}", letter);
        new_letters.push(letter);
    }

    println!("{:?}", new_letters)
}

fn main() {
    greet_world();
    penguins();
    cereal();
    race();
    letters_mine();
}
