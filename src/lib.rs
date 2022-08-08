use std::io;


pub fn get_player_input() -> u8 {
    loop {
        // initial instructions to player
        println!("Input your guess of rock: 1, paper: 2 or scissors: 3");

        // obtain player input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input_integer: u8 = input
            .trim()
            .parse()
            .unwrap_or_default();

        // check if valid value
        match input_integer {
            1|2|3 => break input_integer,
            _ => continue,
        }
    }
}

pub fn determine_winner(p: &u8, c: &u8) {
    match (p,c) {
        (1,1) => println!("Tie!"), // player rock, computer rock
        (1,2) => println!("You lose!"), // player rock, computer paper
        (1,3) => println!("You win!"), // player rock, computer scissors

        (2,1) => println!("You win!"), // player paper, computer rock
        (2,2) => println!("Tie!"), // player paper, computer paper
        (2,3) => println!("You lose!"), //player paper, computer scissors

        (3,1) => println!("You lose!"), // player scissors, computer rock
        (3,2) => println!("You win!"), // player scissors, computer paper
        (3,3) => println!("Tie!"), // player scissors, computer scissors

        _ => panic!("Unable to determine winner")
    }
}