use std::collections::VecDeque;

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct GameState {
    pub snake: VecDeque<Position>,  // meilleur solution pour le déplacement du serpent
    pub fruit: Position,
    pub direction: Direction,
}

