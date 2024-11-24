mod cards;
mod player;
mod dealer;
mod blackjack;
mod cli;
mod server;

// this was used for testing in the terminal
// still here for easy switch to the cli version and debugging
// fn main() {
//     //blackjack::test_game();
//     cli::play_in_cli();
// }

#[macro_use] extern crate rocket;
#[launch]
fn rocket() -> _ {
    server::start_server()
}