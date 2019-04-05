use crate::ui::game::*;
use crate::util::display_cards;

#[derive(Clone)]
pub struct Field {
    pub rows: [Vec<Card>; 4],
}

impl Field {
    pub fn new() -> Field {
        Field {
            rows: [vec![], vec![], vec![], vec![]],
        }
    }

    /// 行 row にカード card を置く
    pub fn place(&mut self, row: usize, card: Card) {
        self.rows[row].push(card);
        debug_assert!(self.rows[row].len() <= 6);
    }

    /// 列 row のスコア合計
    pub fn sum(&self, row: usize) -> Score {
        self.rows[row].iter().cloned().map(score).sum()
    }

    /// 列 row を回収して空にする。スコアの合計を返す。
    pub fn gather(&mut self, row: usize) -> Score {
        let sum = self.sum(row);
        self.rows[row].clear();
        sum
    }

    /// 末尾のカードが card の数以下の最大の行を返す。
    pub fn max_lower(&self, card: Card) -> Option<usize> {
        self.rows
            .iter()
            .enumerate()
            .map(|(index, row)| (index, row.last().expect("empty row!")))
            .filter(|(_, &back)| back < card)
            .max_by_key(|(_, &back)| back)
            .map(|(index, _)| index)
    }

    pub fn five(&self, row: usize) -> bool {
        self.rows[row].len() == 5
    }

    pub fn nimmt(&self, row: usize) -> bool {
        self.rows[row].len() == 6
    }

    pub fn print(&self) {
        for row in self.rows.iter() {
            println!("{}", display_cards(row));
        }
    }
}
