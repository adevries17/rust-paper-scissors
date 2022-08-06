use rand::Rng;

#[derive(Debug)]
pub enum Guess {
    Rock,
    Paper,
    Scissors,
}
impl Guess {
    // function to get computer's guess
    pub fn computer_guess() -> Guess {
        let computer_int: usize = rand::thread_rng().gen_range(1..3);
        match &computer_int {
            1 => Guess::Rock,
            2 => Guess::Paper,
            3 => Guess::Scissors,
            _ => panic!()
        }
    }
    // function to convert the player input a guess
    pub fn player_guess(u_guess: String) -> Guess {
        match &*u_guess {
            "rock" => Guess::Rock,
            "paper" => Guess::Paper,
            "scissors" => Guess::Scissors,
            _ => panic!("Not a valid option. Input rock, paper or scissors")
        }
    }
}