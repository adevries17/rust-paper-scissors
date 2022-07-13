use std::io;
use rand::Rng;
use rust_rps::calc_winner;

fn main() {
    // initial instructions
    println!("Welcome to Rust rock paper scissors!");
    println!("Input your choice of rock, paper, or scissors:");
    println!("1: rock, 2: paper, 3: scissors");
    // get user choice
    let mut user_choice: String = String::new();

    io::stdin()
        .read_line(&mut user_choice).expect("Failed to read input");
    let user_int: u8 = user_choice.trim().parse().unwrap();
    
    // get random rock paper scissors
    let computer_int: u8 = rand::thread_rng().gen_range(1..3);

    calc_winner(computer_int, user_int);

}