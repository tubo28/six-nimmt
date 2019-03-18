use crate::ui::game::Card;

pub fn u64_to_array32(seed: u64, dst: &mut [u8; 32]) {
    for i in 0..8 {
        dst[i] = ((seed >> (i * 8)) & 0xFF) as u8;
    }
}

pub fn display_cards(cards: &Vec<Card>) -> String {
    let cards: Vec<String> = cards.iter().map(|card| format!("{:3}", card)).collect();
    format!("[{}]", cards.join("|"))
}
