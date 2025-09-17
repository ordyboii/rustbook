use std::{cmp::Ordering, io};

use rand::Rng;

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: String) -> Self {
        let num = value.trim().parse().expect("Guess needs to be a string");

        if num < 1 || num > 100 {
            panic!("Number should be between 1 and 100");
        }

        Self { value: num }
    }
}

fn main() {
    println!("Guess the number");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line");

        let guess = Guess::new(guess);

        println!("You guessed: {}", guess.value);

        match guess.value.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
