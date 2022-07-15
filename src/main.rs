mod game_core;
use game_core::*;

fn main(){
    start()
}

pub fn start(){
    const MAX_CHANCES: u8 = 5;
    let random_number = generate_random_number();

    let result = run_game(MAX_CHANCES, random_number, receive_number);
    render_result(result);
}
fn generate_random_number() -> u8 {
    0
}

fn receive_number() -> u8 {
    0
}

fn render_result(result: Result<u8, u8>) {
    match result {
        Ok(chances) => println!("You win!! Chances: {chances}"),
        Err(_) => println!("You lose!!"),
    }
}