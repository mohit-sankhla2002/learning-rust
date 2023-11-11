#[derive(Debug, PartialEq)]
struct Point {
    x: i32, 
    y: i32
}


fn main() {
    let p1 = Point {
        x: 1, 
        y: 4
    };

    let p2 = Point {
        x: 1, 
        y: 4
    };

    let p3 = Point {
        x: 10, 
        y: 20
    };

    // printing any point in debug mode

    // println!("{:?}", p1);

    // Gives this error -> `Point` doesn't implement `Debug`
    // the trait `Debug` is not implemented for `Point`

    println!("{:?}", p1);

    // println!("{}", p1 == p2);

    /* 
        we get this error
        binary operation `==` cannot be applied to type `Point`rustcClick for full compiler diagnostic
        main.rs(31, 20): Point
        main.rs(31, 26): Point
        main.rs(2, 1): an implementation of `PartialEq` might be missing for `Point 
    */

    println!("{}", p1 == p2);
    println!("{}", p2 == p3);

}
