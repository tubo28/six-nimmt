use super::game::*;
use super::field::*;
use rand::prelude::*;
use std::cell::RefCell;

#[derive(Clone)]
pub struct RandomAI {
    name: String,
    used: [bool; 105], // TODO: 内部的に 0..103 にする？
    rng: RefCell<StdRng>,
}

impl RandomAI {
    pub fn new(name: String, seed: u8) -> RandomAI {
        let seed = [seed; 32];
        RandomAI {
            name: name,
            used: [false; 105],
            rng: RefCell::new(StdRng::from_seed(seed)),
        }
    }

    /// 思考して使うカードを選択する
    /// TODO: 乱数じゃなくてちゃんと書く
    pub fn choose_card(&mut self, _turn: usize, _field: &Field, cards: &Vec<Card>) -> Card {
        use rand::seq::SliceRandom;
        let mut rng = self.rng.borrow_mut(); // TODO
        let selected = cards[..].choose(&mut *rng).cloned().expect("no card");
        selected
    }

    /// 置けないときに回収する列を選ぶ
    /// TODO: ランダムじゃなくてまじめにやる
    pub fn choose_gather_row(&mut self, _turn: usize, _choosed_cards: &Vec<Card>, _field: &Field) -> usize {
        let mut rng = self.rng.borrow_mut();
        rng.gen_range(0, 3)
    }
}
