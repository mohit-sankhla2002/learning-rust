// struct Product {
//     name: String, 
//     category: ProductCategory, 
//     price: f32, 
//     in_stock: bool
// } 
// enum ProductCategory {
//     Books, 
//     Clothing,
//     Electronics
// } 

// impl Product {
//     fn new(name: String, category: ProductCategory, price: f32) -> Product{
//         Product { name: name, category: category, price: price, in_stock: true }
//     }
// }

// enums in rust are powerful and can carry data with it 

enum Command {
    Undo, 
    Redo, 
    AddText(String),
    MoveCursor(i32, i32), 
    Replace {
        from: String, 
        to: String
    }
}

impl Command {
    fn serialize(&self) -> String {
        String::from("JSON String")
    }
}

fn main() {
    // let book = Product::new(String::from("Harry Potter"), ProductCategory::Books, 32.22);
    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("Hello World!"));
    let cmd3 = Command::MoveCursor(32, 32);
    let cmd4 = Command::Replace { from: String::from("Here"), to: String::from("There") };

    let json_string = cmd4.serialize();
    
}