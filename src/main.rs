mod ui;

fn main() {
    println!("Hello, world!");
    let mut seed = [0u8; 32];
    seed[0] = 28;
    let mut gm = ui::GameManager::new(seed);
    let players: Vec<String> = vec!["Alpha", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot"].iter().map(|s| s.to_string()).collect();
    let cards = (1..=104).collect();
    gm.initialize(&players[..], &cards);
    gm.run();
}
