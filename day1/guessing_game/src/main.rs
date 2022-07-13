use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess The Number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The Secret Number is: {}", secret_number);

    loop {
        println!("Please Input Your Guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read Line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
