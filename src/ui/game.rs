use crate::ai::ai::AI;
use crate::ui::field::*;
use crate::ui::player::*;
use crate::util::u64_to_array32;

use rand::prelude::*;
// TODO: 構造体でくるむ？
pub type Card = u8;
pub type Score = u8;

/// カードのスコアを求める
pub fn score(card: Card) -> Score {
    // // 生成関数
    // use six_nimmt::ui::game::score;
    // println!("let score_table = [");
    // for i in 0..=104 {
    //     if i != 0 && i % 10 == 0 { println!("") }
    //     print!("{:-3}, ", score(i));
    // }
    // println!("\n];");

    // match card {
    //     55 => 7,
    //     11 | 22 | 33 | 44 | 66 | 77 | 88 | 99 => 5,
    //     10 | 20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 | 100 => 3,
    //     5 | 15 | 25 | 35 | 45 | 65 | 75 | 85 | 95 => 2,
    //     _ => 1,
    // }

    const SCORE_TABLE: [Score; 105] = [
        0, 1, 1, 1, 1, 2, 1, 1, 1, 1, 3, 5, 1, 1, 1, 2, 1, 1, 1, 1, 3, 1, 5, 1, 1, 2, 1, 1, 1, 1,
        3, 1, 1, 5, 1, 2, 1, 1, 1, 1, 3, 1, 1, 1, 5, 2, 1, 1, 1, 1, 3, 1, 1, 1, 1, 7, 1, 1, 1, 1,
        3, 1, 1, 1, 1, 2, 5, 1, 1, 1, 3, 1, 1, 1, 1, 2, 1, 5, 1, 1, 3, 1, 1, 1, 1, 2, 1, 1, 5, 1,
        3, 1, 1, 1, 1, 2, 1, 1, 1, 5, 3, 1, 1, 1, 1,
    ];
    debug_assert!(1 <= card && card <= 104);
    SCORE_TABLE[card as usize]
}

pub struct GameManager {
    pub field: Field,
    pub used_cards: Vec<Vec<Card>>,
    pub players: Vec<Player>,
    pub ais: Vec<Box<dyn AI>>,
}

// TODO: 4, 6, 10 などのマジックナンバーを const にする
impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            field: Field::new(),
            used_cards: Vec::new(),
            players: Vec::new(),
            ais: Vec::new(),
        }
    }

    /// カードをプレイヤーに10枚配り、フィールドに4枚置く
    pub fn initialize(&mut self, seed: u64, players: Vec<Box<dyn AI>>, cards: Vec<Card>) {
        let player_number = players.len();
        assert!(player_number * 10 + 4 <= cards.len());

        // メンバの初期化
        self.field = Field {
            rows: [
                Vec::with_capacity(6),
                Vec::with_capacity(6),
                Vec::with_capacity(6),
                Vec::with_capacity(6),
            ],
        };

        self.players = vec![Player::new(); player_number];

        self.ais = players;

        // フィールドとプレイヤーにカードを配る
        let n = player_number;
        // カードをシャッフルしてプレイヤーに配る
        // assign[i] == p ならば i 番目のカードをプレイヤー p にわたす
        // n <= p < n + 4 ならフィールドの p - n 番目の列に置く
        // p == n + 4 なら山札に残す
        let mut assign = Vec::new();
        for i in 0..n * 10 {
            assign.push(i / 10);
        }
        for i in 0..4 {
            assign.push(n + i);
        }
        for _ in 0..(cards.len() - n * 10 - 4) {
            assign.push(n + 4)
        }
        debug_assert_eq!(cards.len(), assign.len());

        let mut seed_arr = [0; 32];
        u64_to_array32(seed, &mut seed_arr);
        let mut rng = StdRng::from_seed(seed_arr);
        assign.shuffle(&mut rng);

        for (&p, &c) in assign.iter().zip(cards.iter()) {
            if p < n {
                self.players[p].cards.push(c)
            } else if p < n + 4 {
                self.field.rows[p - n].push(c);
            }
        }

        self.field.rows.sort();
        for player in self.players.iter_mut() {
            player.cards.sort();
        }
    }

    /// ラウンドを 10 回回す。
    pub fn run(&mut self) {
        self.print();
        for i in 1..=10 {
            // TODO: log 系の crate を使う
            println!("第{}ターン", i);
            self.go_round(i);
            self.print();
            println!("======================================================");
        }
    }

    /// 1ラウンドを回す。
    /// 各プレイヤーがカードを選び、列への配置、列の回収、スコアの更新を行う。
    pub fn go_round(&mut self, turn: usize) {
        let mut view = self.view_on_choose_card(turn);

        // 各プレイヤーはカードを選ぶ
        // Vec<(player id, Card)>
        let mut moves = Vec::new();
        let mut chosed_cards = Vec::new();

        for (id, (player, ai)) in self.players.iter().zip(self.ais.iter_mut()).enumerate() {
            view.my_cards = player.cards.clone();
            let card = ai.choose_card(&view);
            // TODO: ログ出力をchoose_cardの実装に移す
            println!("プレイヤー{}がカード{}を選びました", id, card);
            moves.push((id, card));
            chosed_cards.push(card);
        }

        // プレイヤーは選んだカードの数が小さい順に行動する
        moves.sort_by_key(|&(_, card)| card);
        for &(player, card) in moves.iter() {
            self.consume_card(turn, player, card, &chosed_cards);
            self.field.print();
        }
    }

    fn consume_card(
        &mut self,
        turn: usize,
        player_index: usize,
        card: Card,
        choosed_cards: &Vec<Card>,
    ) {
        let mut view = self.view_on_choose_gather_row(turn, choosed_cards);

        println!("プレイヤー{}がカード{}を使います", player_index, card);
        let row = self.field.max_lower(card);
        let row = if let Some(row) = row {
            // 置くべき列に置く
            if self.field.five(row) {
                // 置くと6枚になるので5枚回収する
                self.players[player_index].score += self.field.gather(row);
            }
            row
        } else {
            // 置ける列がなかったので回収する列を選ぶ
            view.my_cards = self.used_cards[player_index].clone();
            let row = self.ais[player_index].choose_gather_row(&view);
            self.players[player_index].score += self.field.gather(row);
            row
        };
        // カードを置いて手札から削除する
        self.field.place(row, card);
        assert_eq!(self.field.rows[row].last(), Some(&card));
        let pos = self.players[player_index]
            .cards
            .iter()
            .position(|&c| c == card)
            .expect("no such card");
        let removed = self.players[player_index].cards.remove(pos);
        debug_assert_eq!(removed, card);
    }

    /// my_cards が空なことに注意
    /// AI にわたすときにそのプレイヤーのものに置き換えること
    fn view_on_choose_card(&self, turn: usize) -> ViewOnChoosingCard {
        let mut all_used_cards = Vec::new();
        for cards in self.used_cards.iter() {
            for &card in cards.iter() {
                all_used_cards.push(card)
            }
        }
        all_used_cards.shrink_to_fit();

        ViewOnChoosingCard {
            turn: turn,
            field: self.field.clone(),
            scores: self.players.iter().map(|p| p.score).collect(),
            my_cards: vec![],
            used_cards: all_used_cards,
        }
    }

    /// my_cards が空なことに注意
    /// AI にわたすときにそのプレイヤーのものに置き換えること
    fn view_on_choose_gather_row(
        &self,
        turn: usize,
        choosed_cards: &Vec<Card>,
    ) -> ViewOnGatheringRow {
        let mut all_used_cards = Vec::new();
        for cards in self.used_cards.iter() {
            for &card in cards.iter() {
                all_used_cards.push(card)
            }
        }
        all_used_cards.shrink_to_fit();

        ViewOnGatheringRow {
            turn: turn,
            field: self.field.clone(),
            scores: self.players.iter().map(|p| p.score).collect(),
            my_cards: vec![],
            used_cards: all_used_cards,
            choosed_cards: choosed_cards.clone(),
        }
    }

    pub fn print(&self) {
        println!("プレイヤー:");
        for player in self.players.iter() {
            player.print();
        }
        println!("フィールド:");
        self.field.print();
    }
}

pub struct ViewOnChoosingCard {
    pub turn: usize,
    pub field: Field,
    pub scores: Vec<Score>,
    pub my_cards: Vec<Card>,
    pub used_cards: Vec<Card>,
}

// まともな名前にする
pub struct ViewOnGatheringRow {
    pub turn: usize,
    pub field: Field,
    pub scores: Vec<Score>,
    pub my_cards: Vec<Card>,
    pub used_cards: Vec<Card>,
    pub choosed_cards: Vec<Card>,
}
