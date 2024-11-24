use std::sync::Arc;
use std::sync::Mutex;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State, Request, Response};
use serde::{Deserialize, Serialize};

use crate::blackjack::{Game, GameStatus, Action};
use crate::dealer::Dealer;
use crate::rocket;
use crate::player::Player;
use crate::cards::{self, Card};

pub struct GlobalState {
    player: Player,
    shoe: Vec<Card>,
    game: Game,
}

impl GlobalState {
    pub fn new() -> Self {
        let number_of_decks = 8; // commonly used number of decks for playing Blackjack
        let new_player = Player::new(10_000);
        let new_shoe = cards::generate_shoe(number_of_decks);

        GlobalState {
            player: new_player.clone(),
            shoe: new_shoe.clone(),
            game: Game::new(new_player, new_shoe, number_of_decks),
        }
    }
}

// define data schema sent to the frontend
#[derive(Serialize)]
pub struct GameData {
    dealer: Dealer,
    player: Player,
    bets: i32,
    cards_remaining: i32,
    game_status: GameStatus,
}

impl GameData {
    pub fn new(
        dealer: Dealer,
        player: Player,
        bets: i32,
        shoe: Vec<Card>,
        game_status: GameStatus,
    ) -> Self {
        GameData {
            dealer,
            player,
            bets,
            cards_remaining: shoe.len() as i32,
            game_status,
        }
    }
}

// define actions possible on the frontend
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'r> {
    action: &'r str,
}

// define message for betting amount
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Betting {
    amount: i32,
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "PUT, POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Max-Age", "86400"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

pub fn start_server() -> Rocket<Build> {
    rocket::build()
        .attach(CORS)
        .mount(
            "/",
            routes![index, start_game, init, action, simulate_dealer, end],
        )
        .manage(Arc::new(Mutex::new(GlobalState::new())))
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/init")]
pub fn init(state: &State<Arc<Mutex<GlobalState>>>) -> Json<GameData> {
    // send inital data to the frontend when first started
    let game_data = state.lock().unwrap().game.clone();
    Json(GameData::new(
        game_data.dealer,
        game_data.player,
        game_data.bets,
        game_data.shoe,
        GameStatus::Initalized,
    ))
}

#[post("/startGame", data = "<betting>")]
pub fn start_game(
    state: &State<Arc<Mutex<GlobalState>>>,
    betting: Json<Betting>,
) -> Json<GameData> {
    // save the results from previous game
    let player_data = state.lock().unwrap().game.player.clone();
    let shoe_data = state.lock().unwrap().game.shoe.clone();

    state.lock().unwrap().player = player_data.clone();
    state.lock().unwrap().shoe = shoe_data.clone();

    // reset game
    let mut new_game = Game::new(player_data.clone(), shoe_data.clone(), state.lock().unwrap().game.number_of_decks);

    // start game
    let new_game_status = new_game.start(betting.amount);
    state.lock().unwrap().game = new_game.clone();

    // send data to frontend
    // send only the first card of the dealer to the frontend (i.e. leave the second card face down)
    if new_game_status != GameStatus::DealerWon {
        let first_dealer_card = state.lock().unwrap().game.dealer.hand.first().unwrap().clone();
        let mut modified_dealer = new_game.dealer.clone();
        modified_dealer.hand = vec![first_dealer_card];
        Json(GameData::new(
            modified_dealer,
            new_game.player.clone(),
            new_game.bets.clone(),
            new_game.shoe.clone(),
            new_game_status,
        ))
    } else {

        Json(GameData::new(
            new_game.dealer.clone(),
            new_game.player.clone(),
            new_game.bets.clone(),
            new_game.shoe.clone(),
            new_game_status,
        ))
    }
}

#[post("/action", data = "<message>")]
pub fn action(
    state: &State<Arc<Mutex<GlobalState>>>,
    message: Json<Message<'_>>,
) -> Json<GameData> {
    let new_game_status: GameStatus; // default value

    match message.action {
        "Hit" => new_game_status = state.lock().unwrap().game.play_action(Action::Hit),
        "Stand" => new_game_status = state.lock().unwrap().game.play_action(Action::Stand),
        "Double" => new_game_status = state.lock().unwrap().game.play_action(Action::Double),
        "Split" => unimplemented!(),
        _ => panic!("unknown command."),
    }

    let new_game_data = state.lock().unwrap().game.clone();

    // only the first card of the dealer is sent, since the player is not finished with his turn and the dealers second card is face down
    let first_dealer_card = state.lock().unwrap().game.dealer.hand.first().unwrap().clone();
        let mut modified_dealer = new_game_data.dealer.clone();
        modified_dealer.hand = vec![first_dealer_card];
        Json(GameData::new(
            modified_dealer,
            new_game_data.player.clone(),
            new_game_data.bets.clone(),
            new_game_data.shoe.clone(),
            new_game_status,
        ))
}

#[get("/simulateDealer")]
pub fn simulate_dealer(state: &State<Arc<Mutex<GlobalState>>>) -> Json<GameData> {
    let new_game_status = state.lock().unwrap().game.play_dealers_turn();

    let new_game_data = state.lock().unwrap().game.clone();
    Json(GameData::new(
        new_game_data.dealer,
        new_game_data.player,
        new_game_data.bets,
        new_game_data.shoe,
        new_game_status,
    ))
}

#[post("/end", data = "<message>")]
pub fn end(state: &State<Arc<Mutex<GlobalState>>>, message: Json<Message<'_>>) -> Json<GameData> {
    match message.action {
        "PlayerWon" => state.lock().unwrap().game.end_game(GameStatus::PlayerWon),
        "DealerWon" => state.lock().unwrap().game.end_game(GameStatus::DealerWon),
        "Draw" => state.lock().unwrap().game.end_game(GameStatus::Draw),
        _ => panic!("unknown command."),
    }

    // update player and shoe
    let game_data = state.lock().unwrap().game.clone();
    state.lock().unwrap().player = game_data.player.clone();
    state.lock().unwrap().shoe = game_data.shoe.clone();

    Json(GameData::new(
        game_data.dealer,
        game_data.player,
        game_data.bets,
        game_data.shoe,
        GameStatus::Initalized,
    )) // TODO: change GameStatus to actual status (currently just a placeholder)
}
