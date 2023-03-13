use rand::Rng;
use std::{cmp::Ordering, io, num::ParseIntError};

fn main() {
    let maximum_try = 7;
    let mut num_try = 0;
    let secret_number = get_rand();

    println!("Guess the number!");

    loop {
        if num_try >= maximum_try {
            println!("GAME OVER");
            return;
        }

        println!("Input your guess.");

        let input = parse_input();

        match input {
            Err(_e) => {
                println!("It's not a number.");
                continue;
            }
            Ok(guess) => match guess.cmp(&secret_number) {
                Ordering::Less => println!("secret number is greater than {}", guess),
                Ordering::Greater => println!("secret number is lesser than {}", guess),
                Ordering::Equal => {
                    println!("secret number is {}! Congratulations!", guess);
                    return;
                }
            },
        }
        num_try += 1;
    }
}

fn parse_input() -> Result<i32, ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read.");

    input.trim().parse::<i32>()
}

fn get_rand() -> i32 {
    rand::thread_rng().gen_range(1..100)
}
