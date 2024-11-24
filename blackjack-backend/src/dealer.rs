use serde::Serialize;

use crate::{cards::{self, generate_shoe, Card}, player::Player};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Dealer {
    pub hand: Vec<Card>
}

impl Dealer {
    pub fn new() -> Self {
        Dealer { hand: Vec::new() }
    }

    pub fn draw_card(&mut self, shoe: &mut Vec<Card>, number_of_decks: i8, player_ref: &mut Player, card_is_visible: bool) -> () {
        let mut card = shoe.pop();
        
        // if shoe is empty, refill with new decks
        if card == None {
            shoe.append(&mut generate_shoe(number_of_decks));
            card = shoe.pop();
        }

        // update card count stat
        if card_is_visible {
            player_ref.stats.update_card_count(card.clone().unwrap());
        }

        self.hand.push(card.unwrap());
    }

    pub fn inital_draw(&mut self, shoe: &mut Vec<Card>, number_of_decks: i8, player_ref: &mut Player) -> i8 {
        self.draw_card(shoe, number_of_decks, player_ref, true);
        self.draw_card(shoe, number_of_decks, player_ref, false);

        cards::hand_value(&self.hand)
    }
}