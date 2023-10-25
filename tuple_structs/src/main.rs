fn main() {
    // these are just normal tuples (no enforcing at all)
    let rgb_color = (255, 0, 0); 
    let cmyk_color = (0, 43, 255, 244);

    // tuple structs
    struct RGB(i32, i32, i32);
    struct CMYK(i32, i32, i32, i32);

    let rgb_new = RGB(255, 255, 255);
    let cmyk_new =CMYK(255, 255, 255, 255);

    // unit like structs
    // gist link: https://gist.github.com/mohit-sankhla2002/24c417afa8754b09980cebbe65f33b43
    struct MyStruct;
}
