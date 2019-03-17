use crate::ui::game::Card;

pub fn u64_to_seed_arr(seed: u64, dst: &mut [u8; 32]) {
    for i in 0..8 {
        dst[i] = ((seed >> (i * 8)) & 0xFF) as u8;
    }
}

pub fn display_card_list(cards: &Vec<Card>) -> String {
    let cards: Vec<String> = cards.iter().map(|card| format!("{:3}", card)).collect();
    format!("[{}]", cards.join("|"))
}
