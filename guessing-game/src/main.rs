use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");

        let secret_number = rand::thread_rng().gen_range(1..=100); // generating a random number in the range [1, 100]
        println!("The secret number is: {}", secret_number);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to take input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        }; // Converting a string to integer
        println!("You guessed: {}", guess);

        // Comparing two numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You guess it right!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
