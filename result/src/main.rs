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
    let query_string = format!("SELECT name FROM USERS WHERE user_id = {user_id}");
    let db_result = query_db(query_string);
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String>{
    if query.is_empty() {
        Ok(String::from("Mohit"))
    } else {
        Err(String::from("Something's wrong with the db"))
    }
}

// enum Result<T, E> {
//     Ok<T>:
//     Err<E>
// }

