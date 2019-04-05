use crate::ui::game::Card;
use crate::ui::game::StateView;
use crate::ui::game::StateView2;

pub trait AI {
    /// プレイヤー名を取得する
    fn name(&self) -> String;

    /// 状況 (turn, field, cards) を見てカードを選ぶ
    fn choose_card(&mut self, view: &StateView) -> Card;

    /// 状況 (turn, choose_cards (各プレイヤーが選んだカード)) を見てどの列を回収するか選ぶ
    fn choose_gather_row(&mut self, view: &StateView2) -> usize;
}
