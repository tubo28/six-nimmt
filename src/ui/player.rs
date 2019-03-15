use super::game::*;

#[derive(Clone)]
pub struct Player {
    pub score: Score,
    pub cards: Vec<Card>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            score: 0,
            cards: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("score: {:2}, cards: {:?}", self.score, self.cards);
    }
}
