use super::ai::AI;
use super::field::Field;
use super::game::Card;
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

    fn choose_card(&mut self, turn: usize, field: &Field, cards: &Vec<Card>) -> Card {
        println!("第{}ターン", turn);
        field.print();
        println!("あなたのカード: {:?}", cards);
        print!("出すカードを入力してください: ");
        stdout().flush().unwrap();
        read_valid_card_loop(cards)
    }

    fn choose_gather_row(
        &mut self,
        turn: usize,
        choosed_cards: &Vec<Card>,
        field: &Field,
    ) -> usize {
        println!("第{}ターン", turn);
        field.print();
        println!(
            "他のプレイヤーが出したカード: {:?}",
            choosed_cards
        );
        print!(
            "カードを出せる列がありません。回収する列を選んでください: "
        );
        stdout().flush().unwrap();
        read_valid_card_loop(&[0, 1, 2, 3])
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

/// 標準入力から整数を 1 行読み込めるまで読み続ける
fn read_loop<T: FromStr>() -> T {
    read().unwrap_or_else(|| read_loop())
}

/// valids に含まれる値が読み込まれるまで読み続ける
fn read_valid_card_loop<T: FromStr + Eq>(valids: &[T]) -> T {
    let card = read_loop();
    let pos = valids.iter().position(|x| x == &card);
    pos.map(|_| card)
        .unwrap_or_else(|| read_valid_card_loop(valids))
}
