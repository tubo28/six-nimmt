use crate::ai::ai::AI;
use crate::ui::field::*;
use crate::ui::game::*;
use crate::util::u64_to_array32;
use rand::prelude::*;
use std::cell::RefCell;

#[derive(Clone)]
pub struct RandomAI {
    name: String,
    used: [bool; 105], // TODO: 内部的に 0..103 にする？
    rng: RefCell<StdRng>,
}

impl RandomAI {
    pub fn new(name: String, seed: u64) -> RandomAI {
        let mut seed_arr = [0u8; 32];
        u64_to_array32(seed, &mut seed_arr);
        RandomAI {
            name: name,
            used: [false; 105],
            rng: RefCell::new(StdRng::from_seed(seed_arr)),
        }
    }
}

impl AI for RandomAI {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn choose_card(&mut self, _turn: usize, _field: &Field, cards: &Vec<Card>) -> Card {
        use rand::seq::SliceRandom;
        let mut rng = self.rng.borrow_mut(); // TODO
        let selected = cards[..].choose(&mut *rng).cloned().expect("no card");
        selected
    }

    fn choose_gather_row(
        &mut self,
        _turn: usize,
        _choosed_cards: &Vec<Card>,
        _field: &Field,
    ) -> usize {
        let mut rng = self.rng.borrow_mut();
        rng.gen_range(0, 3)
    }
}
