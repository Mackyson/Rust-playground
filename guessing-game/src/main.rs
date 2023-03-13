use std::{io, num::ParseIntError};

fn main() {
    let _maximum_try = 5;
    println!("Guess the number!");

    println!("Input your guess.");

    let input = input();

    match input {
        Ok(guess) => {
            println!("you guessed the number is {}", guess);
        }
        Err(_e) => {
            println!("It's not a number.");
        }
    }
}
fn input() -> Result<i32, ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read.");

    input.strip_suffix("\n").unwrap().parse::<i32>()
}
