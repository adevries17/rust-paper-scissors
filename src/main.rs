use std::io;
use rand::Rng;

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

    // compare and decide the winner
    // rock = 1, paper = 2, scissors = 3
    if computer_int == user_int {
        println!("Draw!")
    } else if user_int == 1 as u8 && computer_int == 3 as u8 {
        println!("You win!")
    } else if user_int == 2 as u8 && computer_int == 1 as u8 {
        println!("You win!")
    } else if user_int == 3 as u8 && computer_int == 2 as u8 {
        println!("You win!")
    } else if user_int > 3 as u8 || user_int <= 0 {
        println!("Enter 1, 2 or 3")
    } else {
        println!("You lose!")
    }

}
