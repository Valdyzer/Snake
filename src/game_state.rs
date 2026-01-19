use std::collections::VecDeque;
use crate::movement::{Position, Direction};
use rand::Rng;
use crate::interface::{GRID_WIDTH,GRID_HEIGHT};

pub struct GameState {
    pub snake: VecDeque<Position>,  // meilleure solution pour le déplacement du serpent
    pub food: Position,
    pub direction: Direction,
    pub new_direction: Direction,
    pub game_over: bool,
    pub score: u32
}

impl GameState {
    pub fn new() -> Self {

        // Le serpent commence avec trois blocs
        let mut snake = VecDeque::new();
        snake.push_back(Position::new(10, 10));
        snake.push_back(Position::new(9, 10));
        snake.push_back(Position::new(8, 10));

        // État initial de la partie
        Self {
            snake,
            food: Position::new(15, 15),
            direction: Direction::Right,
            new_direction: Direction::Right,
            game_over: false,
            score: 0,
        }
    }
}