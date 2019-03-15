use super::ai::AI;
use super::field::Field;
use super::game::Card;

#[derive(Clone)]
pub struct CLIPlayer {
    name: String,
}

impl CLIPlayer {
    pub fn new(name: String, _seed: u8) -> CLIPlayer {
        println!("random seed is not use");
        CLIPlayer {
            name: name,
        }
    }
}

impl AI for CLIPlayer {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn choose_card(&mut self, _turn: usize, _field: &Field, cards: &Vec<Card>) -> Card {
        unimplemented!();
    }

    fn choose_gather_row(&mut self, _turn: usize, _choosed_cards: &Vec<Card>, _field: &Field) -> usize {
        unimplemented!();
    }

}
