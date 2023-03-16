use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub enum Guess {
    Rock,
    Paper,
    Scissors,
}

impl Distribution<Guess> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Guess {
        match rng.gen_range(0..=2) {
            0 => Guess::Rock,
            1 => Guess::Paper,
            _ => Guess::Scissors,
        }
    }
}

pub fn get_player_guess() -> Guess {
    loop {
        println!("Input your guess of rock, paper or scissors");
        // reserve memory for the string that will be entered
        // by the player, store in the buffer, parse save as string slice
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: String = input.trim().parse().expect("Failed to parse input");
        let input = input.as_str();

        match input {
            "rock" | "r" => return Guess::Rock,
            "paper" | "p" => return Guess::Paper,
            "scissors" | "s" => return Guess::Scissors,
            _ => println!("Not a valid guess! Guess again!"),
        }
    }
}

pub fn determine_winner(p: &Guess, c: &Guess) {
    match (p, c) {
        (Guess::Rock, Guess::Rock) => println!("Tie!"), // player rock, computer rock
        (Guess::Rock, Guess::Paper) => println!("You lose!"), // player rock, computer paper
        (Guess::Rock, Guess::Scissors) => println!("You win!"), // player rock, computer scissors

        (Guess::Paper, Guess::Rock) => println!("You win!"), // player paper, computer rock
        (Guess::Paper, Guess::Paper) => println!("Tie!"),    // player paper, computer paper
        (Guess::Paper, Guess::Scissors) => println!("You lose!"), //player paper, computer scissors

        (Guess::Scissors, Guess::Rock) => println!("You lose!"), // player scissors, computer rock
        (Guess::Scissors, Guess::Paper) => println!("You win!"), // player scissors, computer paper
        (Guess::Scissors, Guess::Scissors) => println!("Tie!"), // player scissors, computer scissors
    }
}

pub fn get_player_rounds() -> usize {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read lines");

    let input = input.trim().parse().expect("Failed to parse input");

    input
}
