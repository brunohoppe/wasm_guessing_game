
pub fn run_game(total_chances: u8, random_number: u8, receive_number: fn() -> u8) -> Result<u8, u8> {
    let mut won_game = false;
    let mut chances = total_chances;

    while can_run(chances, won_game) {
        let user_number = receive_number();
        won_game = verify_win_conditions(random_number)(user_number);
        chances = calculate_chances(chances);
    }
    if won_game {
        return Ok(chances);
    }
    Err(chances)
}

fn calculate_chances(chances: u8) -> u8 {
    chances - 1
}
fn verify_win_conditions(random_number: u8) -> impl Fn(u8) -> bool {
    move |user_number: u8| user_number == random_number
}
fn can_run(chances: u8, won_game: bool) -> bool {
    chances > 0 && !won_game
}

#[cfg(test)]
mod test_game_logic {
    use super::*;

    #[test]
    fn test_run_game() {
        todo!()
    }
}