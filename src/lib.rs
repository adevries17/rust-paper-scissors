use rand::Rng;


#[derive(Debug,PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}
impl Choice {
    pub fn computer_choice() -> Choice {
        let computer_int = rand::thread_rng().gen_range(1..3);
        match &computer_int {
            1 => Choice::Rock,
            2 => Choice::Paper,
            3 => Choice::Scissors,
            _ => panic!()
        }
    }
    pub fn user_choice(u_choice: String) -> Choice {
        match &*u_choice {
            "rock" => Choice::Rock,
            "paper" => Choice::Paper,
            "scissors" => Choice::Scissors,
            _ => panic!()
        }
    }
}