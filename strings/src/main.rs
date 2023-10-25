fn main() {
    // let s1 = "Hello World!";
    // let _s2 = String::from("Hello World!");
    // let _s3 = s1.to_string();
    // let s4 = s1.to_owned();
    // let s5 = &s4;

    // println!("{s5}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    s.replace_range(.., "baz"); // similar to splice in javascript
    println!("{}", s);
}
