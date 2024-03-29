use crate::ai::ai::AI;
use crate::ui::game::Card;
use crate::ui::game::ViewOnChoosingCard;
use crate::ui::game::ViewOnGatheringRow;
use crate::util::display_cards;
use std::cmp::Eq;
use std::io::Write;
use std::io::{stdin, stdout};
use std::str::FromStr;

#[derive(Clone)]
pub struct CLIPlayer {
    name: String,
}

impl CLIPlayer {
    pub fn new(name: String) -> CLIPlayer {
        CLIPlayer { name: name }
    }
}

impl AI for CLIPlayer {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn choose_card(&mut self, view: &ViewOnChoosingCard) -> Card {
        view.field.print();
        println!("あなたのカード: {}", display_cards(&view.my_cards));
        read_valid_or_retry(&view.my_cards, "出すカードを入力してください")
    }

    fn choose_gather_row(&mut self, view: &ViewOnGatheringRow) -> usize {
        println!("第{}ターン", view.turn);
        view.field.print();
        println!("他のプレイヤーが出したカード: {:?}", view.choosed_cards);
        read_valid_or_retry(
            &[0, 1, 2, 3],
            "カードを出せる列がありません。回収する列を選んでください",
        )
    }
}

/// 標準入力から 1 行読み込む
fn read_line() -> String {
    let mut line = String::new();
    stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");
    line.trim().chars().collect()
}

/// 標準入力から T 型の値を 1 行読み込む
fn read<T: FromStr>() -> Option<T> {
    let line = read_line();
    line.parse::<T>().ok()
}

/// valids に含まれる値が読み込まれるまで読み続ける
fn read_valid_or_retry<T: FromStr + Eq>(valids: &[T], message: &str) -> T {
    loop {
        print!("{}: ", message);
        stdout().flush().unwrap();
        if let Some(val) = read() {
            if let Some(_) = valids.iter().position(|x| x == &val) {
                return val;
            }
        }
    }
}
