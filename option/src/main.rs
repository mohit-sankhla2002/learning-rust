fn main() {
    let username = get_username(1);
    // match username {
    //     Some(name) => println!("{name}"),
    //     None => println!("No user found")
    // } // this is one way to do it 

    // another concise way 
    if let Some(name) = username {
        println!("{name}");
    }
}

fn get_username(user_id: i32) -> Option<String> {
    // get username from database
    let db_result = String::from("Mohit");
    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}

// enum Option<T> {
//     None, 
//     Some(T)
// }

