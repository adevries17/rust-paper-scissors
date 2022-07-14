use std::io;
use rust_rps::Choice;


fn main() {
    // initial instructions
    println!("Welcome to Rust rock paper scissors!");
    println!("Input your choice of rock, paper, or scissors:");
    println!("rock, paper, scissors");
    // get user input
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    let user_string: String = user_input.trim().parse().unwrap();

    let c_choice = Choice::computer_choice();
    let u_choice = Choice::user_choice(user_string);

    println!("{:?}", c_choice);
    println!("{:?}", u_choice);
}