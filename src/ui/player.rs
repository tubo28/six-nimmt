use crate::ui::game::*;
use crate::util::display_card_list;

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
        println!(
            "score: {:-3}, cards: {}",
            self.score,
            display_card_list(&self.cards)
        );
    }
}
