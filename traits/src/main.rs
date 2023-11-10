struct Car {
    make: String,
    model: String, 
    year: u8
}

struct Truck {
    make: String,
    model: String, 
    year: u8
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck....")
    }
}

// now we have to implement a functionality for both the car and the truck as parking

// in classical inheritance, we would have implemented a struct known as a vehicle and inherited it's properties into both car and truck from vehicle

// to do that we'll use traits -> similar to interfaces in java just like an abstract class :)

trait Park {
    fn park(&self);
}

impl Park for Car {
    fn park(&self) {
        println!("parking..")
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("something heavy parking..")
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painted: {color}")
    }
}

impl Paint for Car {
    fn paint(&self, color: String) {
        println!("painted car: {color}")
    }
}

struct House {
    house_number: u8
}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting {} with color: {color}", self.house_number);
    }
}

fn main() {
    let t1 = Truck {
        make: "Hello".to_owned(), 
        model: "2021".to_owned(),
        year: 21
    };

    let c1 = Car {
        make: "123".to_owned(),
        model: "25".to_owned(), 
        year: 25
    };

    c1.park();
    t1.park();
}