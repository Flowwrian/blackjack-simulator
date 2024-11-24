use inquire::{error::InquireError, prompt_u32, Select};

use crate::{
    blackjack::{Action, GameCliVersion, GameStatus},
    cards::{self, Card},
    player::Player,
};

pub fn play_in_cli() -> () {
    // define player actions
    let player_options: Vec<&str> = vec!["Hit", "Stand", "Double", "Split", "Quit"];

    // initialize global objects
    let mut player: Player = Player::new(10_000);
    let mut shoe: Vec<Card> = cards::generate_shoe(8);

    println!("Welcome to this game of Blackjack!");

    'game: loop {
        // initialize game
        let mut game: GameCliVersion<'_, '_> = GameCliVersion::new(&mut player, &mut shoe, 8);
        let mut game_status: GameStatus;

        println!("{} cards remain", game.shoe.len());
        println!("Your current balance is: {}", game.player.balance);
        let betting_amount = prompt_u32("How much do you want to bet:");

        // betting phase
        match betting_amount {
            Ok(amount) => {
                println!("You are betting {:?}", amount);
                game_status = game.start(amount as i32);
            }
            Err(_) => {
                println!("Quiting the game");
                break;
            }
        }

        // checking the dealers cards
        if game_status == GameStatus::DealerWon {
            println!("The dealer has 21! You lost.");
            game.end_game(GameStatus::DealerWon);
            continue;
        }

        println!(
            "The dealer has one card open: {}",
            game.dealer.hand.first().unwrap()
        );
        println!("Your card is: {}", game.player.hand.first().unwrap());

        // player chooses his action
        'decision_phase: loop {
            if game_status != GameStatus::Ongoing {
                break 'decision_phase;
            }

            let ans: Result<&str, InquireError> =
                Select::new("Select an action", player_options.clone()).prompt();

            match ans {
                Ok(decision) => match decision {
                    "Hit" => {
                        game_status = game.play_action(Action::Hit);
                        println!("You drew a card: {}", game.player.hand.last().unwrap());
                        println!(
                            "Your hand value is: {}",
                            cards::hand_value(&game.player.hand)
                        );
                    }
                    "Stand" => {
                        game_status = game.play_action(Action::Stand);
                        println!("You finished your turn.");
                        println!(
                            "Your hand value is: {}",
                            cards::hand_value(&game.player.hand)
                        );
                        break 'decision_phase;
                    }
                    "Double" => {
                        game_status = game.play_action(Action::Double);
                        println!("You decided to double.");
                        println!(
                            "Your hand value is: {}",
                            cards::hand_value(&game.player.hand)
                        );
                        break 'decision_phase;
                    }
                    "Split" => {
                        game_status = game.play_action(Action::Split);
                    }
                    "Quit" => break 'game,
                    _ => panic!("This should not happen!"),
                },
                Err(_) => panic!("There was an error!"),
            };
        }

        // dealers turn
        if game_status == GameStatus::DealerWon {
            println!("The dealers hand is: {:?}", &game.dealer.hand);
            println!("The dealer won!");
            game.end_game(game_status);
            continue;
        } else {
            game_status = game.play_dealers_turn();
            println!("The dealers hand is: {:?}", &game.dealer.hand);
            println!("{:?}", game_status);
            game.end_game(game_status);
        }
    }
}
