mod ui;

fn main() {
    println!("Hello, world!");
    let mut seed = [0u8; 32];
    seed[0] = 28;
    let mut gm = ui::GameManager::new(seed);
    let nplayer = 6;
    let cards = (1..=104).collect();
    gm.initialize(nplayer, &cards);
    gm.run();
}
