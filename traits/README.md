# Trait Bounds

Now what if we wanted to define a function that paints the object that we pass in to red color

**See the code for better understanding**

``` rust
    fn paint<T>(object: &T) {
        object.paint("red".to_owned())
    }
```

*this function takes in an object and paints it red -> But this gives a severe error*

**T** can be any concrete type such as a String or an i32 or anything else. We are not certain that that object that we pass in will always implement a paint method.

##### Hence we'll have to bound T to methods that implement the Paint trait
--- 
#### There are 3 ways you can do that: - 
1. 
```
fn paint_red1<T: Paint>(object: &T) {
    object.paint("red".to_owned())
}
```

2. 
```
fn paint_red2(object: &impl Paint) {
    object.paint("red".to_owned())
}
```

3. 
```
fn paint_red3<T>(object: &T) where T: Paint {
    object.paint("red".to_owned())
}
```

## Super Traits
Just like a struct relies upon a trait to get some properties, there are some traits which also rely on other traits. The traits that are relied on are called as the super traits

## Trait Objects
In monomorphization where we used to convert the code from type dependent to specific at compile time as the compiler knew where we had to place the types. If we don't have that type of sophistication where the compiler does not know what will the type of data that we are going to get, we need something **dynamic**.

    Box<dyn Paint> // -> Box Pointer understands everything that uses the paint trait

More on this in the rust book -> I have read the article from there only 
