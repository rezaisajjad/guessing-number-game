use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

const START_RANGE: i32 = 0;
const END_RANGE: i32 = 10000;
fn main() {
    println!("Welcome to guessing game");

    let rn: i32 = generate_number();

    loop {
        let guess: i32 = read_number();

        match guess.cmp(&rn) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => println!("Too small!"),
        }
    }
}
fn print_ask_guess() {
    print!("enter your Guess: ");
    io::stdout().flush().unwrap();
}
fn read_number() -> i32 {
    print_ask_guess();
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => read_number(),
    }
}
fn generate_number() -> i32 {
    rand::thread_rng().gen_range(START_RANGE..=END_RANGE)
}
