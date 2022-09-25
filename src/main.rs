use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");
    println!("I'm thinking of a number between 1 and 50");
    
    let mut rand_num = rand::thread_rng().gen_range(1..50);

    loop {
        println!("Please enter your number guess.");

        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        let user_guess: u32 = user_guess.trim().parse().expect("Type a number!");

        if user_guess > rand_num{
            println!("Your number is too high!");
        } else if user_guess < rand_num{
            println!("Your number is too low!");
        } else{
            println!("Thats Correct!");
            break;
        }
    }
}   
