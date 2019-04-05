use crate::ai::ai::AI;
use crate::ui::game::Card;
use crate::ui::game::StateView;
use crate::ui::game::StateView2;

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

    fn choose_card(&mut self, _view: &StateView) -> Card {
        unimplemented!()
    }

    fn choose_gather_row(&mut self, _view: &StateView2) -> usize {
        unimplemented!()
    }
}
