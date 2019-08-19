use crate::ai::ai::AI;
use crate::ui::game::Card;
use crate::ui::game::ViewOnChoosingCard;
use crate::ui::game::ViewOnGatheringRow;

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

    fn choose_card(&mut self, _view: &ViewOnChoosingCard) -> Card {
        unimplemented!()
    }

    fn choose_gather_row(&mut self, _view: &ViewOnGatheringRow) -> usize {
        unimplemented!()
    }
}
