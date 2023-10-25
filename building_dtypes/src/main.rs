// Creating a struct with product 

struct Product {
    name: String, 
    price: f32, 
    in_stock: bool
} // what is the difference between tuples and structs then?

// implementation block
impl Product {
    fn new(name: String, price: f32,) -> Product {
        Product { name: name, price: price, in_stock: true }
    }

    fn display(&self) {
        println!("name: {}", self.name);
        println!("price: {}", self.price);
        println!("in_stock: {}", self.in_stock);
    }

    fn default_sales_tax() -> f32 {
        0.1 // this is an associated function as it does not take self as an argument
    }
    fn calculate_tax(&self) -> f32 {
        self.price * Product::default_sales_tax() // associated functions are called like this 
    }    
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 { // this will return the reciept number
        let name = self.name;
        if self.in_stock == false {
            println!("{name} can't be bought");
        } else {
            println!("{name} was bought");
        }
        12123 
    } // self is dropped here, ownership is deleted inside the block hence the asset owned is defined 
}

fn main() {
    let mut book = Product{
        name: String::from("Book"),
        price: 32.33,
        in_stock: true
    };
    Product::calculate_tax(&book);
    let price = book.price;
    println!("{price}");
    let sales_tax = book.calculate_tax();
    book.set_price(44.0);
    book.buy();
    println!("{sales_tax}");

    let mut vegetable = Product::new(String::from("Carrot"), 23.23,);
    vegetable.display();
}

