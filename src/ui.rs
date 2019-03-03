// TODO: 構造体でくるむ？
type Card = u8;
type Score = u8;

pub struct Field {
    cards: [Vec<Card>; 4]
}

impl Field {
    fn new_empty() -> Field {
        Field {
            cards: [
                Vec::with_capacity(6),
                Vec::with_capacity(6),
                Vec::with_capacity(6),
                Vec::with_capacity(6),
            ]
        }
    }
}

pub fn score(card: Card) -> Score {
    match card {
        55 => 7,
        11 | 22 | 33 | 44 | 66 | 77 | 88 | 99 => 5,
        10 | 20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 | 100 => 3,
        5 | 15 | 25 | 35 | 45 | 65 | 75 | 85 | 95 => 2,
        _ => 1,
    }
}

impl Field {
    pub fn place(&mut self, row: usize, card: Card) {
        self.cards[row].push(card);
        debug_assert!(self.cards[row].len() <= 6);
    }

    pub fn sum(&self, row: usize) -> Score {
        self.cards[row].iter().cloned().map(score).sum()
    }

    pub fn gather(&mut self, row: usize) -> Score {
        let negative = self.sum(row);
        self.cards[row].clear();
        negative
    }

    pub fn get_row(&self, card: Card) -> Option<usize> {
        (0..4).filter(|&i| {
            let back = self.cards[i].last().expect("empty row!");
            *back < card
        }).min_by_key(|&i| self.cards[i].last().expect("empty row!"))
    }

    pub fn check_full(&self, row: usize) -> bool {
        self.cards[row].len() == 6
    }
}

pub struct Player {
    score: Score,
    cards: Vec<Card>,
}

impl Player {
    fn think(&self, others: &Vec<Player>, field: &Field) -> Card /* row */ {
        0 // TODO
    }

    // FIXME: プレイヤーとフィールドのどちらを主体と見るか微妙なのでグローバル関数のほうがいいかも
    fn use_card(&mut self, card: Card, field: &mut Field) {
        let row = field.get_row(card);
        if let Some(r) = row {
            field.place(r, card);
            if field.check_full(r) {
                let s = field.gather(r);
                self.score += s;
            }
        } else {
            // 置ける列がなかったので回収する列を選ぶ
            let gather_row = 0; // TODO
            field.place(gather_row, card);
        }
    }
}

pub struct GameManager {
    field: Field,
    players: Vec<Player>,
}

impl GameManager {
    // ランダムに4枚選びフィールドに置く
    fn init(&mut self, player_number: usize, cards: &Vec<Card>) {
        assert!(player_number * 10 + 4 <= cards.len());

        let n = player_number;
        // カードをシャッフルしてプレイヤーに配る
        self.field = Field::new_empty();
        self.players = (0..n).map(|_| Player { score: 0, cards: vec![] }).collect();

        // assignes[i] == p ならば i 番目のカードをプレイヤー p にわたす
        // n <= p < n + 4 ならフィールドの p - n 番目の列に置く
        // p == n + 4 なら山札に残す
        let mut assignes = Vec::new();
        for i in 0..n*10 {
            assignes.push(i / 10);
        }
        for i in 0..4 {
            assignes.push(n + i);
        }
        for _ in 0..(cards.len() - n*10 - 4) {
            assignes.push(n + 4)
        }

        // TODO: assignes をシャッフル

        for (&p, &c) in assignes.iter().zip(cards.iter()) {
            if p < n {
                self.players[p].cards.push(c)
            } else if p < n + 4 {
                self.field.cards[p - n].push(c);
            }
        }
    }

    fn run(&mut self) {
        for i in 0..10 {
            self.go_round();
        }
    }

    fn go_round(&mut self) {
        let n = self.players.len();
        // Vec<(player id, Card)>
        let mut moves: Vec<(usize, Card)> = self.players.iter()
            .map(|player| player.think(&self.players, &self.field))
            .enumerate()
            .collect();
        moves.sort_by_key(|&(_, card)| card);
        for &(player, card) in moves.iter() {
            self.players[player].use_card(card, &mut self.field);
        }
    }
}
