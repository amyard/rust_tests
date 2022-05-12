use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    //let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables,
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        
        // We switch from an -expect- call to a -match- expression to move from crashing on an error to handling the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);
        
        // Matching the result
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// variables are mutable by default
// &mut guess ---> & indicate this value is reference
