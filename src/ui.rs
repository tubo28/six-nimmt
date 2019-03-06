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
        (0..4)
            .filter(|&i| {
                let back = self.rows[i].last().expect("empty row!");
                *back < card
            })
            .min_by_key(|&i| self.rows[i].last().expect("empty row!"))
    }

    pub fn is_full(&self, row: usize) -> bool {
        self.rows[row].len() == 6
    }

    pub fn print(&self) {
        for row in self.rows.iter() {
            println!("{:?}", row);
        }
    }
}

#[derive(Clone)]
pub struct Player {
    score: Score,
    cards: Vec<Card>,
    rng: RefCell<StdRng>,
}

impl Player {
    fn new(seed: [u8; 32]) -> Player {
        Player {
            score: 0,
            cards: Vec::new(),
            rng: RefCell::new(StdRng::from_seed(seed)),
        }
    }

    pub fn print(&self) {
        println!("score: {:2}, cards: {:?}", self.score, self.cards);
    }

    // 思考して使うカードを選択する
    fn think(&self, _others: &Vec<Player>, _field: &Field) -> Card {
        use rand::seq::SliceRandom;
        let mut rng = self.rng.borrow_mut();
        self.cards[..].choose(&mut *rng).cloned().expect("no card")
    }

    // FIXME: プレイヤーとフィールドのどちらを主体と見るか微妙なのでグローバル関数のほうがいいかも
    fn consume_card(&mut self, card: Card, field: &mut Field) {
        let mut rng = self.rng.borrow_mut();
        let row = field.max_lower(card);
        let row = if let Some(row) = row {
            // 置くべき列に置く
            field.place(row, card);
            if field.is_full(row) {
                self.score += field.gather(row);
            }
            row
        } else {
            // 置ける列がなかったので回収する列を選ぶ
            let row = rng.gen_range(0, 3);
            self.score += field.gather(row); // TODO
            row
        };
        // カードを置いて手札から削除する
        field.place(row, card);
        assert_eq!(field.rows[row].last(), Some(&card));
        let pos = self
            .cards
            .iter()
            .position(|&c| c == card)
            .expect("no such card");
        let removed = self.cards.remove(pos);
        debug_assert_eq!(removed, card);
    }
}

pub struct GameManager {
    field: Field,
    players: Vec<Player>,
    rng: RefCell<StdRng>,
}

// TODO: 4, 6, 10 などのマジックナンバーを const にする
impl GameManager {
    pub fn new(seed: [u8; 32]) -> GameManager {
        GameManager {
            field: Field::new(),
            players: Vec::new(),
            rng: RefCell::new(StdRng::from_seed(seed)),
        }
    }

    /// カードをプレイヤーに10枚配り、フィールドに4枚置く
    pub fn initialize(&mut self, player_number: usize, cards: &Vec<Card>) {
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

        self.players.clear();
        self.players.reserve(player_number);
        for i in 0..player_number {
            let mut seed = [0; 32];
            seed[0] = i as u8;
            self.players.push(Player::new(seed));
        };

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

        let mut rng = self.rng.borrow_mut();
        assign.shuffle(&mut *rng);

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
        }
    }

    /// 1ラウンドを回す。
    /// 各プレイヤーがカードを選び、列への配置、列の回収、スコアの更新を行う。
    pub fn go_round(&mut self) {
        // 各プレイヤーはカードを選ぶ
        // Vec<(player id, Card)>
        let mut moves: Vec<(usize, Card)> = self
            .players
            .iter()
            .map(|player| player.think(&self.players, &self.field))
            .enumerate()
            .collect();
        // プレイヤーは選んだカードの数が小さい順に行動する
        moves.sort_by_key(|&(_, card)| card);
        for &(player, card) in moves.iter() {
            self.players[player].consume_card(card, &mut self.field);
        }
    }

    pub fn print(&self) {
        for player in self.players.iter() {
            player.print();
        }
        self.field.print();
    }
}
