fn type_of<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}

fn main() {
    println!("Hello, world!");

    // * Scalar Types
    // ** Integer Types
    //    8 bit lenght : {i8, u8} ...
    // ** Integer Literals
    //    Number literal:(Example) {Decimal:(98_222); Hex:(0xff);
    //                              Octal:(0o77); Binary(0b1111_0000);
    //                              Byte(u8):(b'A')}

    // ** Floating-Point Types
    let x = 2.0; // f64 - default
    println!("x = {0} ({1})", x, type_of(&x));
    
    let y: f32 = 3.0; // f32
    println!("y = {0} ({1})", y, type_of(&y));

    // * Numeric Operations - Apendix B for a list of operators
    let sum = 5 + 10; // addition

    let difference = 95.5 - 4.3; // subtraction

    let product = 4 * 30; // multiplication

    let quotient = 56.7 / 32.2; // division

    let remainder = 43 % 5; // remainder

    println!("(+,-,*,/,%) ) = {0}, {1}, {2}, {3}, {4}", sum,
             difference, product, quotient, remainder);
    
    // * Boolean Types
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("t = {0} ({1})", t, type_of(&t));
    println!("f = {0} ({1})", f, type_of(&f));
    
    // * Character Type
    let c = 'z'; // char (4 bytes in size)
    let z = 'ℤ'; // unicode thingy
    let heart_eyed_cat = '😻'; // emoji thingy

    println!("c = {0} ({1})", c, type_of(&c));
    println!("z = {0} ({1})", z, type_of(&z));
    println!("emoji = {0} ({1})", heart_eyed_cat, type_of(&heart_eyed_cat));
    
    // * Compound Type
    // ** Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of (x,y,z) is: {},{},{}", x, y, z);

    // * Array
    let a_arr = [1, 2, 3, 4, 5];
    let b_arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a_arr = {:?}", a_arr);
    println!("b_arr = {:?}", b_arr);
    
    let c_arr = [3; 5];
    println!("c_arr = {:?}", c_arr);

    println!("a_arr[0] = {}", a_arr[0]);
}
