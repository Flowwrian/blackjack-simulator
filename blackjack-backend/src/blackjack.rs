use serde::Serialize;

use crate::{
    cards::{self, Card},
    dealer::Dealer,
    player::Player,
};

#[derive(Debug, PartialEq, Serialize)]
pub enum GameStatus {
    Initalized,
    Ongoing,
    PlayerWon,
    PlayerFinished,
    DealerWon,
    Draw,
}

pub enum Action {
    Hit,
    Stand,
    Double,
    Split,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct GameCliVersion<'player, 'shoe> {
    pub dealer: Dealer,
    pub player: &'player mut Player,
    pub bets: i32,
    pub shoe: &'shoe mut Vec<Card>,
    pub number_of_decks: i8,
}

// this implementation was used for the cli version of the game
// it uses &mut Player instead of Player; still here to easily switch back to cli version for debugging
impl<'player, 'shoe> GameCliVersion<'player, 'shoe> {
    pub fn new(
        player: &'player mut Player,
        shoe: &'shoe mut Vec<Card>,
        number_of_decks: i8,
    ) -> Self {
        GameCliVersion {
            dealer: Dealer::new(),
            player,
            bets: 0,
            shoe,
            number_of_decks,
        }
    }

    pub fn start(&mut self, bets: i32) -> GameStatus {
        // set bets
        self.player.balance -= bets;
        self.bets = bets;

        // update betting stats
        self.player.stats.update_average_bet(bets);
        self.player.stats.update_matches_played();

        // dealer draws his first two cards
        self.dealer
            .inital_draw(&mut self.shoe, self.number_of_decks, self.player);

        if cards::hand_value(&self.dealer.hand) == 21 {
            GameStatus::DealerWon
        }
        // player draws his first card
        else {
            self.player.draw_card(&mut self.shoe, self.number_of_decks);
            GameStatus::Ongoing
        }
    }

    pub fn play_action(&mut self, action: Action) -> GameStatus {
        match action {
            Action::Hit => {
                self.player.draw_card(&mut self.shoe, self.number_of_decks);
                if cards::hand_value(&self.player.hand) > 21 {
                    GameStatus::DealerWon
                } else if cards::hand_value(&self.player.hand) == 21 {
                    GameStatus::PlayerWon
                } else {
                    GameStatus::Ongoing
                }
            }
            Action::Stand => GameStatus::PlayerFinished,
            Action::Double => {
                // double bets
                self.player.balance -= self.bets;
                self.bets += self.bets;

                self.player.draw_card(&mut self.shoe, self.number_of_decks);

                // update stats
                self.player.stats.update_average_bet(self.bets);
                self.player.stats.update_times_doubled();

                // check if player won
                if cards::hand_value(&self.player.hand) > 21 {
                    GameStatus::DealerWon
                } else if cards::hand_value(&self.player.hand) == 21 {
                    GameStatus::PlayerWon
                } else {
                    GameStatus::PlayerFinished
                }
            }
            Action::Split => unimplemented!(),
        }
    }

    pub fn play_dealers_turn(&mut self) -> GameStatus {
        // dealer unviels his second card
        self.player
            .stats
            .update_card_count(self.dealer.hand.last().unwrap().clone());

        while cards::hand_value(&self.dealer.hand) < 17 {
            self.dealer
                .draw_card(&mut self.shoe, self.number_of_decks, self.player, true);
        }
        // check if dealer is bust
        if (cards::hand_value(&self.dealer.hand) > 21)
            || (cards::hand_value(&self.dealer.hand) < cards::hand_value(&self.player.hand))
        {
            GameStatus::PlayerWon
        } else if cards::hand_value(&self.dealer.hand) == cards::hand_value(&self.player.hand) {
            GameStatus::Draw
        } else {
            GameStatus::DealerWon
        }
    }

    pub fn end_game(&mut self, state: GameStatus) -> () {
        if state == GameStatus::PlayerWon {
            self.player.balance += self.bets * 2;

            // update stats
            self.player.stats.update_average_win(self.bets);
        } else if state == GameStatus::Draw {
            self.player.balance += self.bets;
            self.player.stats.update_average_win(0);
        } else if state == GameStatus::DealerWon {
            // update stats
            self.player.stats.update_average_win(-self.bets);
        }

        // reset hands
        self.dealer.hand = Vec::new();
        self.player.hand = Vec::new();
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Game {
    pub dealer: Dealer,
    pub player: Player,
    pub bets: i32,
    pub shoe: Vec<Card>,
    pub number_of_decks: i8,
}

impl Game {
    pub fn new(player: Player, shoe: Vec<Card>, number_of_decks: i8) -> Self {
        Game {
            dealer: Dealer::new(),
            player,
            bets: 0,
            shoe,
            number_of_decks,
        }
    }

    pub fn start(&mut self, bets: i32) -> GameStatus {
        // set bets
        self.player.balance -= bets;
        self.bets = bets;

        // update betting stats
        self.player.stats.update_average_bet(bets);
        self.player.stats.update_matches_played();

        // dealer draws his first two cards
        self.dealer
            .inital_draw(&mut self.shoe, self.number_of_decks, &mut self.player);

        if cards::hand_value(&self.dealer.hand) == 21 {
            GameStatus::DealerWon
        }
        // player draws his first card
        else {
            self.player.draw_card(&mut self.shoe, self.number_of_decks);
            GameStatus::Ongoing
        }
    }

    pub fn play_action(&mut self, action: Action) -> GameStatus {
        match action {
            Action::Hit => {
                self.player.draw_card(&mut self.shoe, self.number_of_decks);
                if cards::hand_value(&self.player.hand) > 21 {
                    GameStatus::DealerWon
                } else if cards::hand_value(&self.player.hand) == 21 {
                    GameStatus::PlayerWon
                } else {
                    GameStatus::Ongoing
                }
            }
            Action::Stand => GameStatus::PlayerFinished,
            Action::Double => {
                // double bets
                self.player.balance -= self.bets;
                self.bets += self.bets;

                self.player.draw_card(&mut self.shoe, self.number_of_decks);

                // update stats
                self.player.stats.update_average_bet(self.bets);
                self.player.stats.update_times_doubled();

                // check if player won
                if cards::hand_value(&self.player.hand) > 21 {
                    GameStatus::DealerWon
                } else if cards::hand_value(&self.player.hand) == 21 {
                    GameStatus::PlayerWon
                } else {
                    GameStatus::PlayerFinished
                }
            }
            Action::Split => unimplemented!(),
        }
    }

    pub fn play_dealers_turn(&mut self) -> GameStatus {
        // dealer unviels his second card
        self.player
            .stats
            .update_card_count(self.dealer.hand.last().unwrap().clone());
        while cards::hand_value(&self.dealer.hand) < 17 {
            self.dealer
                .draw_card(&mut self.shoe, self.number_of_decks, &mut self.player, true);
        }
        // check if dealer is bust
        if (cards::hand_value(&self.dealer.hand) > 21)
            || (cards::hand_value(&self.dealer.hand) < cards::hand_value(&self.player.hand))
        {
            GameStatus::PlayerWon
        } else if cards::hand_value(&self.dealer.hand) == cards::hand_value(&self.player.hand) {
            GameStatus::Draw
        } else {
            GameStatus::DealerWon
        }
    }

    pub fn end_game(&mut self, state: GameStatus) -> () {
        if state == GameStatus::PlayerWon {
            self.player.balance += self.bets * 2;
            // update stats
            self.player.stats.update_average_win(self.bets);
        } else if state == GameStatus::Draw {
            self.player.balance += self.bets;
            self.player.stats.update_average_win(0);
        } else if state == GameStatus::DealerWon {
            self.player.stats.update_average_win(-self.bets);
        }

        // reset hands
        self.dealer.hand = Vec::new();
        self.player.hand = Vec::new();
    }
}