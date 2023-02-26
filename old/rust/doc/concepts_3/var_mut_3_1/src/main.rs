fn main() {
    
    println!("\nMutability\n");

    // let x = 5; // immutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const - constant // let - variable
    // const MAX_POINTS: u32 = 100_000; // example of constant declaration

    println!("\nShadowing\n");
    
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {} (after let x = x + 1)", x);
    let x = x * 2;
    println!("The value of x is: {} (after let x = x * 2)", x);

    /* Shadowing is different from marking a variable as mut, because we’ll get
       a compile-time error if we accidentally try to reassign to this variable
       without using the let keyword 
     */

    // let mut spaces = "   ";
    // spaces = spaces.len(); // error[E0308]: mismatched types
    // We're not allowed to mutate a variable's type
}
