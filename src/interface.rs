// Dimensions de la grille
pub const GRID_WIDTH: i32 = 20;
pub const GRID_HEIGHT: i32 = 20;

use dioxus::prelude::*;
use crate::game_state::GameState;
use crate::movement::Position;

#[component]
// Permet d'avoir une vue sur l'état actuel du jeu
pub fn Board(state: ReadOnlySignal<GameState>) -> Element {
    let state = state.read();
    
    
    let grid_style = format!(
        "display: grid; \
         grid-template-columns: repeat({}, 20px); \
         grid-template-rows: repeat({}, 20px); \
         gap: 1px; \
         background-color: #484848ff; \
         border: 2px solid #767676ff; \
         width: fit-content;"
    , GRID_WIDTH, GRID_HEIGHT);

    
    

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background-color: #222; color: white; font-family: sans-serif;",
            h1 { "Snake Game" }
            div { "Score: {state.score}" }
            div {
                 style: "{grid_style}",
                 
            }
        }
    }
}
