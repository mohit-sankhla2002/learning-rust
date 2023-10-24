fn main() {
    let tweet = String::from("This is my tweet and this is very long!");

    // strings are nothing but a series of bytes and in rust, the bytes are utf8 encoded
    let trimmed_tweet : &str = &tweet[..20]; // this is a string slice 
    println!("{trimmed_tweet}");

    // So from here we can deduce that there are two types of strings in rust, one is String and another one is &str
    let functional_trimmed_tweet = trim_tweet(&tweet);
    println!("From Function: {functional_trimmed_tweet}");

    // deref coercion

    let tweet2 = "This is a very very long tweet";
    let trimmed_tweet_2 = trim_tweet(tweet2);
    println!("{trimmed_tweet_2}");
    // using slices on arrays 
    let arr = [1,2,3,4,5,6];
    let arr_slice = &arr[..3];
    println!("{:?}", arr_slice);
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20] // give the first 20 characters of the string
}
