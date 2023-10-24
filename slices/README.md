## Slices: - 
```
fn main() {

	let tweet = String::from("This is my tweet and this is very long!");
	
	  
	
	// strings are nothing but a series of bytes and in rust, the bytes are utf8 encoded
	
	let trimmed_tweet : &str = &tweet[..20]; // this is a string slice
	
	println!("{trimmed_tweet}");
	
	// So from here we can deduce that there are two types of strings in rust, one is String and another one is &str

}
```

### String types: - 
1. String 
	- growable, heap allocated string (UTF8 Encoded)
2. str (pronounced as ster)
	- Immutable sequence of UTF-8 bytes somewhere in the memory (stack, heap and static)
	- Handle behind a reference (&str) because length of the sequence is unknown at the compile time 

```
fn main() {
	let tweet = String::from("This is my tweet and this is very long!");
	
	// strings are nothing but a series of bytes and in rust, the bytes are utf8 encoded
	
	let trimmed_tweet : &str = &tweet[..20]; // this is a string slice
	
	println!("{trimmed_tweet}");
	
	// So from here we can deduce that there are two types of strings in rust, one is String and another one is &str
	let functional_trimmed_tweet = trim_tweet(&tweet);
	println!("From Function: {functional_trimmed_tweet}");
}

  

fn trim_tweet(tweet: &String) -> &str {
	&tweet[..20] // give the first 20 characters of the string
}
```

What happens we we pass a string slice into a function that accepts only reference to strings 
For Eg: 
```
fn main() {
	let tweet2 = "This is a very very long tweet";
	let trimmed_tweet_2 = trim_tweet(tweet2); // there will be an error here 
}

fn trim_tweet(tweet: &String) -> &str {
	&tweet[..20] // give the first 20 characters of the string
}
```

To solve this, we take advantage of a property of rust: - (Deref Coercion)
```
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
	println!({"trimmed_tweet_2"});
}

  

fn trim_tweet(tweet: &str) -> &str {

&tweet[..20] // give the first 20 characters of the string

}
```

More on deref coercions: - 
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/deref-coercions.html


### Using Slicing on Arrays: - 
```
	let arr = [1,2,3,4,5,6];

	let arr_slice = &arr[..3];

	println!("{:?}", arr_slice);
```

this prints out `[1, 2, 3]`
## Debug Formatting: - 
https://doc.rust-lang.org/std/fmt/trait.Debug.html#:~:text=Debug%20should%20format%20the%20output,see%20the%20module%2Dlevel%20documentation.

`println!("{:?}", arr)`
