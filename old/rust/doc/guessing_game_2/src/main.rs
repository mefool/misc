use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable variable declaration, bound
                                       // to a empty instance of a String

        io::stdin()
            .read_line(&mut guess)          // & - reference, immutabble by default
            .expect("Failed to read line"); // handling potential failures
                                        // warning if not handled

        println!("You guessed: {}", guess); // {} - placeholders
        
        let guess: u32 = match guess.trim().parse() {
            // .trim() - remove whitespace and \n
            // .parse() - parses string into some kind of number (u32 here)
            Ok(num) => num,     // if the string is able to be parsed
            Err(_) => continue, // if not ...
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
