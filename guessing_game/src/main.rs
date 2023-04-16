use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // 1st exercise
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generating a random number between 1 and 100

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // Basically initializing a new empty string that is mutable

    // Getting User Input

    io::stdin()
        .read_line(&mut guess) // Pass a mutable reference of "guess"
        .expect("Failed to read line"); // Handling error

    // One way of handling errors
    let guess: u32 = guess.trim().parse().expect("Please type a number!"); // After getting the User input, we trim the string and we parse it (in this case to a Number) and expect a message if an error is thrown

    // Second way, this one only works inside loops

    /* let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    }; */

    println!("You guessed {guess}"); // Just printing "guess"

    // Comparing the guess with the secret number generated

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
