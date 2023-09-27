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

fn main() {
    greet_world();
    penguins();
}
