use rps_rs::*;

fn main() {
    println!("Welcome to Rust rock paper scissors!");

    // get player guess
    let player_guess = get_player_guess();

    // get computer guess
    let computer_guess: Guess = rand::random();

    println!("Computer chose: {:?}", computer_guess);
    println!("You chose: {:?}", player_guess);

    // determine winner
    determine_winner(&player_guess, &computer_guess);
}
