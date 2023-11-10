// struct BrowserCommand {
//     name: String, 
//     payload: String
// }

struct BrowserCommand<T> {
    name: String, 
    payload: T
}

// Creating an implementation block for generic structs

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name: name, payload: payload }
    }

    fn get_payload(&self) -> &T {
        return &self.payload;
    }

}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}
fn main() {
    
    // let cmd3 = BrowserCommand<i32>{"zoom".to_owned(), 200}; // here it'll give an error saying that 200 is an i32 and we have specified in the struct 

    let cmd3 = BrowserCommand {
        name: "zoom".to_owned(), 
        payload: 200
    }; // tis works now just fine

    let cmd4 = BrowserCommand::new("Hash".to_owned(), "1223234324");

    

}
