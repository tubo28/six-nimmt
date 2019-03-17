use crate::ai::ai::AI;
use crate::ui::field::Field;
use crate::ui::game::Card;

#[derive(Clone)]
pub struct MonteCalroAI {
    name: String,
}

impl MonteCalroAI {
    pub fn new(name: String) -> MonteCalroAI {
        MonteCalroAI { name: name }
    }
}

impl AI for MonteCalroAI {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn choose_card(&mut self, _turn: usize, _field: &Field, _cards: &Vec<Card>) -> Card {
        unimplemented!()
    }

    fn choose_gather_row(
        &mut self,
        _turn: usize,
        _choosed_cards: &Vec<Card>,
        _field: &Field,
    ) -> usize {
        unimplemented!()
    }
}
