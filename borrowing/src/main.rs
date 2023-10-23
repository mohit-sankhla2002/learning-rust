fn main() {
    let mut s1 = String::from("Rust");
    let r1 = &s1; // this is a immutable reference 
    print_string(r1);
    let r2 = &mut s1; // non lexical lifetimes 
    add_to_string(r2);
    print_string(r2);
}   

fn add_to_string(r1: &mut String){
    r1.push_str(" is Awesome"); // automatic dereferencing
}

fn print_string(r1: &String) {
    println!("{r1}"); // automatic dereferencing
}

// Note: You cannot return a reference from a function due to one obvious reason -> lifetime of heap in a block of code
