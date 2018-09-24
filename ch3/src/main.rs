fn main() {
    // constants
    const MAX_POINTS: u32 = 100_000;

    // variables
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is now: {}", x);

    //////////////////////////////////////////////////////
    // scalar types - represents a single value
    // integers, floating-point numbers, booleans, characters

    // integer types
    const SIGNED_INT: i32 = -100;
    const UNSIGNED_INT: u32 = 100;

    // floating-point types
    let _x = 2.0; // f64
    let _y: f32 = 3.0;

    // char
    let _c = 'z';

    //////////////////////////////////////////////////////
    // compound types
    
    // tuple type
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("Middle value from tuple: {}", _tup.1);

    // array type
    let a = [1, 2, 3, 4, 5];
}
