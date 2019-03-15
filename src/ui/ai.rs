use super::field::Field;
use super::game::Card;

pub trait AI {
    fn name(&self) -> String;
    fn choose_card(&mut self, turn: usize, field: &Field, cards: &Vec<Card>) -> Card;
    fn choose_gather_row(&mut self, turn: usize, choosed_cards: &Vec<Card>, field: &Field)
        -> usize;
}
