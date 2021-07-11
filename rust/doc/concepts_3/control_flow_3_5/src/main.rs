fn main() {
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // switch probably does
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // "else if" can clutter the code
    // chapter 6 will introduce "match" branching construct

    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // loop {
    //    println!("again!");
    //}

    // while cycle
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for cycle
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
