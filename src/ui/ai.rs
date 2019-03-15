use super::game::Card;
use super::field::Field;

pub trait AI {
    // TODO: すべての AI がシードを使うわけではないので取らない形にすべき
    fn new(name: String, seed: u8) -> Self;
    fn name(&self) -> String;
    fn choose_card(&mut self, turn: usize, field: &Field, cards: &Vec<Card>) -> Card;
    fn choose_gather_row(&mut self, turn: usize, choosed_cards: &Vec<Card>, field: &Field) -> usize;
}
