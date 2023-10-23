use num::integer::sqrt;


fn main() {
    let numbers = [36, 25, 49, 3, 64, 16, 9];
    let _x = get_prime(numbers);
    println!("{}", _x);
}

fn get_prime(arr: [i32; 7]) -> i32 {
    let mut i = 0;
    'outer: loop {
        let mut n = 2;
        'inner: loop {
            if arr[i] % 2 == 0 {
                if arr[i] == 2 {
                    break 'outer;
                }
                i+=1;
                break 'inner;
            }

            if n >= sqrt(arr[i]) {
                break 'outer;
            }

            n+=1;
        }
    }
    arr[i]
}
