use dioxus::prelude::*;

mod movement; // Charge le module
mod interface;
mod game_state;

use movement::{Position, Direction}; // Importe les structures
use interface::{GRID_HEIGHT, GRID_WIDTH};
use game_state::GameState;
use interface::Board;


fn main() {
    dioxus::launch(App);    // Lance l'app
}

#[component]
fn App() -> Element {
    let mut game_state = use_signal(|| GameState::new());

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

