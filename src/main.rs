use std::io;
use rps_rs::Guess;

fn main() {
    // initial instructions to player
    println!("Welcome to Rust rock paper scissors!");
    println!("Input your guess of (r)ock, (p)aper, or (s)cissors:");
    println!("rock, paper, scissors");
    // get player input
    let mut player_input:String = String::new();
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read input");

    let player_string:String = player_input.trim().parse().expect("This is not a valid guess");

    let c_guess: Guess = Guess::computer_guess();
    let u_guess: Guess = Guess::player_guess(player_string);

    println!("Computer guess: {:?}", c_guess);
    println!("player guess: {:?}", u_guess);
}