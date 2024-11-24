use core::fmt;
use std::slice::Iter;
use rand::thread_rng;
use rand::seq::SliceRandom;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum Color {
    Clubs, // ♣
    Diamonds, // ♦
    Hearts, // ♥
    Spades // ♠
}

impl Color {
    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 4] = [
            Color::Clubs, 
            Color::Diamonds, 
            Color::Hearts, 
            Color::Spades
            ];
        COLORS.iter()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Clubs => write!(f, "♣"),
            Color::Diamonds => write!(f, "♦"),
            Color::Hearts => write!(f, "♥"),
            Color::Spades => write!(f, "♠"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Value {
    pub fn iterator() -> Iter<'static, Value> {
        static VALUES: [Value; 13] = [
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
            Value::Ace
        ];
        VALUES.iter()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Two => write!(f, "2"),
            Value::Three => write!(f, "3"),
            Value::Four => write!(f, "4"),
            Value::Five => write!(f, "5"),
            Value::Six => write!(f, "6"),
            Value::Seven => write!(f, "7"),
            Value::Eight => write!(f, "8"),
            Value::Nine => write!(f, "9"),
            Value::Ten => write!(f, "10"),
            Value::Jack => write!(f, "J"),
            Value::Queen => write!(f, "Q"),
            Value::King => write!(f, "K"),
            Value::Ace => write!(f, "A"),
        }
    }
}


#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Card {
    pub color: Color,
    pub value: Value,
    pub numeric_value: i8,
}

impl Card {
    pub fn new(color: Color, value: Value) -> Self {
        Self { 
            color, 
            value, 
            numeric_value: match value {
                Value::Two => 2,
                Value::Three => 3,
                Value::Four => 4,
                Value::Five => 5,
                Value::Six => 6,
                Value::Seven => 7,
                Value::Eight => 8,
                Value::Nine => 9,
                Value::Ten => 10,
                Value::Jack => 10,
                Value::Queen => 10,
                Value::King => 10,
                Value::Ace => 11, // remember: this value can be 11 or 1
            }
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value,self.color)
    }
}

pub fn generate_shoe(decks: i8) -> Vec<Card> {
    let mut shoe: Vec<Card> = Vec::new();

    // number of decks
    for _ in 0..decks {
        // iterate colors
        for color in Color::iterator() {
            // iterate values
            for value in Value::iterator() {
                let new_card = Card::new(color.clone(), value.clone());
                shoe.push(new_card);
            }
        }
    }
    shoe.shuffle(&mut thread_rng());
    shoe
}

pub fn hand_value(hand: &Vec<Card>) -> i8 {
    let mut hand_value = i8::from(0);
    let mut aces: Vec<Card> = Vec::new();

    for card in hand.iter() {
        // sum up all non-ace cards
        if card.value != Value::Ace {
            hand_value += card.numeric_value;
        } else {
            aces.push(card.clone());
        }
    }

    // add all aces (i.e. 11 or 1)
    for ace in aces.iter() {
        if (hand_value + ace.numeric_value) > 21 {
            hand_value += 1;
        } else {
            hand_value += 11
        }
    }

    hand_value
}