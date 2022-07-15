pub fn start(){
    const MAX_CHANCES: u8 = 5;

    let random_number = generate_random_number();
    let mut won_game = false;
    let mut chances = MAX_CHANCES;

    while chances > 0 && !won_game {
        let user_number = receive_number();
        won_game = user_number == random_number;
        chances -= 1;
    }
    if won_game {
        println!("You win!!")
    } else {
        println!("You lose!!")
    }


}
fn generate_random_number() -> u8 {
    0 
}

fn receive_number() -> u8 {
    0 
}

#[cfg(test)]
mod test_game_logic {
    use super::*;

    #[test]
    fn test_random_number() {
        assert_eq!(generate_random_number(), 0)
    }
}