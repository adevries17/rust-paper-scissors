use rps_rs::*;

fn main() {
    println!("Welcome to Rust rock paper scissors!");
    println!("How many rounds would you like to play?");
    let player_rounds = get_player_rounds();
    let mut rounds: usize = 0;

    while rounds < player_rounds {
        // get player guess
        let player_guess = get_player_guess();

        // get computer guess
        let computer_guess: Guess = rand::random();

        println!("Computer chose: {:?}", computer_guess);
        println!("You chose: {:?}", player_guess);

        // determine winner
        determine_winner(&player_guess, &computer_guess);

        rounds += 1;
    }
}
