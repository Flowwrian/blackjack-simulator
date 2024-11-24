use rocket::serde::Serialize;

use crate::cards::{generate_shoe, Card};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct PlayerStats { 
    all_bets: Vec<i32>,
    all_wins: Vec<i32>,
    matches_played: i32,
    average_bet: i32,
    times_doubled: i32,
    average_win: i32,
    card_count: i32 // value, used in card counting to determine odds of high cards
}

impl PlayerStats {
    pub fn new() -> Self {
        PlayerStats {
            all_bets: Vec::new(),
            all_wins: Vec::new(),
            matches_played: 0,
            average_bet: 0,
            times_doubled: 0,
            average_win: 0,
            card_count: 0
        }
    }

    pub fn update_matches_played(&mut self) -> () {
        self.matches_played += 1;
    }

    pub fn update_average_bet(&mut self, new_bet: i32) -> () {
        let total_times_betted = self.matches_played + self.times_doubled;
        self.all_bets.push(new_bet);
        self.average_bet = self.all_bets.iter().sum::<i32>() / (total_times_betted + 1);
    }

    pub fn update_times_doubled(&mut self) -> () {
        self.times_doubled += 1;
    }

    pub fn update_average_win(&mut self, new_win: i32) -> () {
        self.all_wins.push(new_win);
        self.average_win = self.all_wins.iter().sum::<i32>() / self.matches_played;
    }

    pub fn update_card_count(&mut self, new_card: Card) -> () {
        if new_card.numeric_value <= 6 {
            self.card_count += 1;
        } else if new_card.numeric_value >= 10 {
            self.card_count -= 1;
        }
    }

}


#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(crate="rocket::serde")]
pub struct Player {
    pub balance: i32,
    pub hand: Vec<Card>, // TODO: Replace with Hand struct to allow splitting hands
    pub stats: PlayerStats
}

impl Player {
    pub fn new(balance: i32) -> Self {
        Player{ balance, hand: Vec::new(), stats: PlayerStats::new() }
    }

    pub fn draw_card(&mut self, shoe: &mut Vec<Card>, number_of_decks: i8) -> () {
        let mut card = shoe.pop();

        // if shoe is empty, refill with new decks
        if card == None {
            shoe.append(&mut generate_shoe(number_of_decks));
            card = shoe.pop();
        }

        // update card count stat
        self.stats.update_card_count(card.clone().unwrap());

        self.hand.push(card.unwrap());
    }
}