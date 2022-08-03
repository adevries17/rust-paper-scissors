use std::io;
use rand::Rng;

fn main() {
    // initial instructions to player
    println!("Welcome to Rust rock paper scissors!");
    println!("Input your choice of rock, paper, or scissors:");
    println!("rock, paper, scissors");
    // get player input
    let mut player_input:String = String::new();
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read input");

    let player_string:String = player_input.trim().parse().unwrap();

    let c_choice: Choice = Choice::computer_choice();
    let u_choice: Choice = Choice::player_choice(player_string);

    println!("Computer choice: {:?}", c_choice);
    println!("player choice: {:?}", u_choice);
}

#[derive(Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}
impl Choice {
    // function to get computer's choice
    pub fn computer_choice() -> Choice {
        let computer_int: usize = rand::thread_rng().gen_range(1..3);
        match &computer_int {
            1 => Choice::Rock,
            2 => Choice::Paper,
            3 => Choice::Scissors,
            _ => panic!()
        }
    }
    // function to convert the player input a choice
    pub fn player_choice(u_choice: String) -> Choice {
        match &*u_choice {
            "rock" => Choice::Rock,
            "paper" => Choice::Paper,
            "scissors" => Choice::Scissors,
            _ => panic!("Not a valid option. Input rock, paper or scissors")
        }
    }
}