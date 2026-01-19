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

    // Boucle
    use_future(move || async move {
        loop {
            TimeoutFuture::new(GAME_SPEED_MS as u32).await;
            game_state.write().phase();

        }
    });

    rsx! {
        div {
            outline: "none",
            tabindex: "0",
            autofocus: true,
            style: "width: 100vw; height: 100vh; outline: none; overflow: hidden;",
            Board {state : game_state}
        }
    }
}

