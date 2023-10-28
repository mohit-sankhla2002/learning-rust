fn main() {
    // let v: Vec<String> = Vec::new(); // one way
    let mut v = Vec::new();
    v.push("Hello".to_string()); // apne aap string dekhega smajh jaayega
    v.push("How".to_string());
    v.push("Are".to_string());
    v.push("You".to_string());
    v.push("?".to_string());


    // using the vec macro to create vectors
    let _v2 = vec![1, 2, 3, 4];
    let _s = &v[0]; // can panic if the index is out of the bounds of the vector
    // let s = v.remove(0); 

    // to get an option to prevent panicking we use the get method on the vector
    let s1 = v.get(5);
    if let Some(number) = s1 {
        println!("{}", number);
    } else {
        println!("Can't Read Vector");
    }

    for i in &mut v {
        i.push_str("!")
    }

    println!("{:?}", v);

    
}
