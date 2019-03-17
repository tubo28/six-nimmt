use crate::ui::field::Field;
use crate::ui::game::Card;

pub trait AI {
    /// プレイヤー名を取得する
    fn name(&self) -> String;

    /// 状況 (turn, field, cards) を見てカードを選ぶ
    fn choose_card(&mut self, turn: usize, field: &Field, cards: &Vec<Card>) -> Card;

    /// 状況 (turn, choose_cards (各プレイヤーが選んだカード)) を見てどの列を回収するか選ぶ
    fn choose_gather_row(&mut self, turn: usize, choosed_cards: &Vec<Card>, field: &Field)
        -> usize;
}
