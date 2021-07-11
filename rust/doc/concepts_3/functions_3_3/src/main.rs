fn main() {
    println!("Hello, world!");

    println!("Function");
    another_function();

    println!("Function with parameters.");
    another_function_with_parameters(32);

    println!("Function with 2 parameters.");
    another_function_with_two_parameters(32, 64);

    println!("Function with return values. (five)");
    let x = five();
    println!("The value of x is: {}", x);

    println!("Function with return values. (plus_one)");
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameters(x: i32) {
    println!("The calue of x is: {}", x);
}

fn another_function_with_two_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
