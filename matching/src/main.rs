fn main() {
    let age: i32 = 35;

    match age {
        1 => println!("Happy first birthday"),
        13..=19 => println!("You're a teenager"),
        20..=69 => println!("You are an adult"),
        70..=100 => println!("You are going to die soon"),
        101.. => println!("You are a god"),
        // _ => println!("") // works like a else block
        x => println!("You are {x} years old")
    }
    // match expressions are exhaustive => you have to handle every possible combination or possibility 
}
