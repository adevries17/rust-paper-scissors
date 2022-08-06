use std::io;
use rand::Rng;

fn determine_winner(p:&u8, c:&u8) {
    match (p, c) {
        (1,1) => println!("Tie"),
        _ => panic!("Unable to determine winner")
    }
}
fn main() {
    // initial instructions to player
    println!("Welcome to Rust rock paper scissors!");
    println!("Input your guess of rock: 1, paper: 2 or scissors: 3");


    // get player input
    let mut player_input = String::new();

    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read input");
    let player_integer:u8 = player_input
        .trim()
        .parse()
        .expect("Failed to read input");


    // get computer guess
    let computer_guess:u8 = rand::thread_rng().gen_range(1..3);

    println!("Computer chose: {computer_guess}");
    println!("You chose: {player_integer}");


    // determine winner
    determine_winner(&player_integer, &computer_guess);

}