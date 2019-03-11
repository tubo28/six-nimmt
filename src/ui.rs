use rand::prelude::*;
use std::cell::RefCell;

// TODO: 構造体でくるむ？
pub type Card = u8;
pub type Score = u8;

/// カードのスコアを求める
pub fn score(card: Card) -> Score {
    match card {
        55 => 7,
        11 | 22 | 33 | 44 | 66 | 77 | 88 | 99 => 5,
        10 | 20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 | 100 => 3,
        5 | 15 | 25 | 35 | 45 | 65 | 75 | 85 | 95 => 2,
        _ => 1,
    }
}

pub struct Field {
    rows: [Vec<Card>; 4],
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
        self.rows.iter().enumerate().map(|(index, row)| (index, row.last().expect("empty row!")))
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
        for (i, row) in self.rows.iter().enumerate() {
            println!("row {} : {:?}", i, row);
        }
    }
}

#[derive(Clone)]
pub struct Player {
    score: Score,
    cards: Vec<Card>,
}

impl Player {
    fn new() -> Player {
        Player {
            score: 0,
            cards: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("score: {:2}, cards: {:?}", self.score, self.cards);
    }
}

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

pub struct GameManager {
    field: Field,
    players: Vec<Player>,
    ais: Vec<RandomAI>,
}

// TODO: 4, 6, 10 などのマジックナンバーを const にする
impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            field: Field::new(),
            players: Vec::new(),
            ais: Vec::new(),
        }
    }

    /// カードをプレイヤーに10枚配り、フィールドに4枚置く
    pub fn initialize(&mut self, seed: u8, players: &[RandomAI], cards: &Vec<Card>) {
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

        self.ais = players.iter().cloned().collect();

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

        let mut rng = StdRng::from_seed([seed; 32]);
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
        for i in 0..10 {
            // TODO: log 系の crate を使う
            println!("{}-th round", i);
            self.go_round();
            self.print();
            println!("===========================");
        }
    }

    /// 1ラウンドを回す。
    /// 各プレイヤーがカードを選び、列への配置、列の回収、スコアの更新を行う。
    pub fn go_round(&mut self) {
        // 各プレイヤーはカードを選ぶ
        // Vec<(player id, Card)>
        let mut moves = Vec::new();
        let mut chosed_cards = Vec::new();
        for (id, (player, ai)) in self.players.iter().zip(self.ais.iter_mut()).enumerate() {
            let card = ai.choose_card(0, &self.field, &player.cards);
            moves.push((id, card));
            chosed_cards.push(card);
        }
        // プレイヤーは選んだカードの数が小さい順に行動する
        moves.sort_by_key(|&(_, card)| card);
        println!("{:?}", moves);
        for &(player, card) in moves.iter() {
            self.consume_card(player, card, &chosed_cards);
            self.field.print();
        }
    }

    fn consume_card(&mut self, player_index: usize, card: Card, choosed_cards: &Vec<Card>) {
        let row = self.field.max_lower(card);
        println!("player {} uses {} -> row {:?}", player_index, card, row);
        let row = if let Some(row) = row {
            // 置くべき列に置く
            if self.field.five(row) {
                // 置くと6枚になるので5枚回収する
                self.players[player_index].score += self.field.gather(row);
            }
            row
        } else {
            // 置ける列がなかったので回収する列を選ぶ
            let row = self.ais[player_index].choose_gather_row(0, choosed_cards, &self.field);
            self.players[player_index].score += self.field.gather(row);
            row
        };
        // カードを置いて手札から削除する
        self.field.place(row, card);
        assert_eq!(self.field.rows[row].last(), Some(&card));
        let pos = self
            .players[player_index].cards
            .iter()
            .position(|&c| c == card)
            .expect("no such card");
        let removed = self.players[player_index].cards.remove(pos);
        debug_assert_eq!(removed, card);
    }

    pub fn print(&self) {
        println!("Players:");
        for player in self.players.iter() {
            player.print();
        }
        println!("Field:");
        self.field.print();
    }
}
