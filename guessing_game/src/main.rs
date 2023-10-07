use rand::Rng;
use std::cmp::Ordering;
use std::io; // bring in the io lib into scope, comes from standard lib

// fn main is the input of the program
fn main() {
    println!("Guess the number!"); // macro print!

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        /*
         ** let statement to create variable example let apples = 5;
         ** in rust variables are immutable by default, to make variable mutable we must assign mut to it
         ** String::new() :: indicates that new() is a associated fun of the String type
         ** an associated fn is a fn that's implemented on a type
         */
        // loop will cause infinite loop
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // adding &mut guess gives read_line the reference, references are immutable by default which is why we do &mut guess rather then &guess
            .expect("Failed to read line"); // read_line returns a Result which is a enum which is a type that can be in multiple states

        let guess: u32 = match guess.trim().parse() {
            // match to match a Result<Ok, Error>
            Ok(num) => num,
            Err(_) => continue,
        };
        // .expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break; // break loop
            }
        }
    }
}
