use std::collections::VecDeque;
use crate::movement::{Position, Direction};
use rand::Rng;
use crate::interface::{GRID_WIDTH,GRID_HEIGHT};

pub struct GameState {
    pub snake: VecDeque<Position>,  // meilleure solution pour le déplacement du serpent
    pub fruit: Position,
    pub direction: Direction,
    pub new_direction: Direction,
    pub game_over: bool,
    pub score: u32
}

impl GameState {
    // Nouvelle partie
    pub fn new() -> Self {

        // Le serpent commence avec trois blocs
        let mut snake = VecDeque::new();
        snake.push_back(Position::new(10, 10));
        snake.push_back(Position::new(9, 10));
        snake.push_back(Position::new(8, 10));

        // État initial de la partie
        Self {
            snake,
            fruit: Position::new(15, 15),
            direction: Direction::Right,
            new_direction: Direction::Right,
            game_over: false,
            score: 0,
        }
    }


    // Toutes les 150 ms 
    pub fn phase(&mut self) {
        if self.game_over {
            return;
        }

        self.direction = self.new_direction;    // Nouvelle direction ? Sinon on garde la même
        let head = self.snake.front().unwrap();
        
        let new_head = match self.direction {
            Direction::Up => Position::new(head.x, head.y - 1),
            Direction::Down => Position::new(head.x, head.y + 1),
            Direction::Left => Position::new(head.x - 1, head.y),
            Direction::Right => Position::new(head.x + 1, head.y),
        };

        if self.check_collision(new_head) {
            self.game_over = true;
            return;
        }

        // Fait avancer la tête 
        self.snake.push_front(new_head);

        if new_head == self.fruit {
            self.score += 1;
        } 
        else {
            self.snake.pop_back();  // Fait avancer le corps
        }
        
    }


    // Changement de direction du serpent
    pub fn set_direction(&mut self, direction: Direction) {

        // ------------------- Empêcher les demi-tours --------------------------- //

        self.new_direction = direction;            
    }


    fn check_collision(&self, pos: Position) -> bool {
        // Contre un mur (on applique les limites)
        if pos.x < 0 || pos.x >= GRID_WIDTH || pos.y < 0 || pos.y >= GRID_HEIGHT {
            return true;
        }

        // Contre le corps du serpent (on vérifie si le corps contient la tête)
        self.snake.contains(&pos)
    }
}