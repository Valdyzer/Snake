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

    let mut cells = Vec::new();

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let pos = Position::new(x, y);
            let is_snake_head = state.snake.front() == Some(&pos);
            let is_snake_body = !is_snake_head && state.snake.contains(&pos);
            let is_fruit = state.fruit == pos;
            
            // Couleur des éléments
            let color = if is_snake_head {
                "green"             // Tête du serpent
            } else if is_snake_body {
                "lightgreen"        // Queue du serpent
            } else if is_fruit {
                "red"               // Fruit
            } else {
                "#2f2d2dff"       // Case vide
            };
            
            // Application à chaque cellule
            cells.push(rsx! {
                div {
                    key: "{x}-{y}",
                    style: "width: 20px; height: 20px; background-color: {color};"
                }
            });
        }
    }
    

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background-color: #222; color: white; font-family: sans-serif;",
            h1 { "Snake Game" }
            div { "Score: {state.score}" }
            div { style: "{grid_style}", {cells.into_iter()} }  // Convertit le vecteur en itérateur

            if state.game_over {
                div { style: "margin-top: 20px; color: red; font-weight: bold; font-size: 1.5rem;", "GAME OVER" }
                div { "Press SPACE to restart" }
            }
        }
    }
}
