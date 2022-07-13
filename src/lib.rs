pub fn calc_winner(c_input: u8, u_input: u8) {
    // compare and decide the winner
    // rock = 1, paper = 2, scissors = 3
    if c_input == u_input {
        println!("Draw!")
    } else if u_input == 1 as u8 && c_input == 3 as u8 {
        println!("You win!")
    } else if u_input == 2 as u8 && c_input == 1 as u8 {
        println!("You win!")
    } else if u_input == 3 as u8 && c_input == 2 as u8 {
        println!("You win!")
    } else if u_input > 3 as u8 || u_input <= 0 {
        println!("Enter 1, 2 or 3")
    } else {
        println!("You lose!")
    }
}