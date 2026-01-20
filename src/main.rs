const GAME_SPEED_MS: u64 = 150;

mod movement; // Charge le module
mod interface;
mod game_state;

use dioxus::prelude::*;
use movement::{Position, Direction}; // Importe les structures
use interface::{GRID_HEIGHT, GRID_WIDTH};
use game_state::GameState;
use interface::Board;
use gloo_timers::future::TimeoutFuture;



fn main() {
    dioxus::launch(App);    // Lance le jeu
}

#[component]
fn App() -> Element {
    let mut game_state = use_signal(|| GameState::new());

    // Boucle Principale
    use_future(move || async move {
        loop {
            TimeoutFuture::new(GAME_SPEED_MS as u32).await;
            game_state.write().phase();

        }
    });

    // Requêtes clavier
    let key_resquest = move |evt: KeyboardEvent| {
        let mut state = game_state.write();
        match evt.key() {
            Key::ArrowUp => state.set_direction(Direction::Up),
            Key::ArrowDown => state.set_direction(Direction::Down),
            Key::ArrowLeft => state.set_direction(Direction::Left),
            Key::ArrowRight => state.set_direction(Direction::Right),
            
            // taper la barre ESPACE pour recommencer la partie
            Key::Character(char) if char == " " => {
                if state.game_over {
                    *state = GameState::new();
                }
            }
            _ => {}
        }
    };


    rsx! {
        div {
            outline: "none",
            tabindex: "0",
            autofocus: true,
            style: "width: 100vw; height: 100vh; outline: none; overflow: hidden;", 
            onkeydown: key_resquest,

            Board {state : game_state}
        }
    }
}

