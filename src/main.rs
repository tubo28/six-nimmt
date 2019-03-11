mod ui;

fn main() {
    println!("Hello, world!");
    let seed = 32;
    let mut gm = ui::GameManager::new();
    let players = vec![
        ui::RandomAI::new("Alpha".to_string(), seed),
        ui::RandomAI::new("Bravo".to_string(), seed),
        ui::RandomAI::new("Charlie".to_string(), seed),
        ui::RandomAI::new("Delta".to_string(), seed),
        ui::RandomAI::new("Echo".to_string(), seed),
        ui::RandomAI::new("Foxtrot".to_string(), seed),
    ];
    let cards = (1..=104).collect();
    gm.initialize(seed, &players[..], &cards);
    gm.run();
}
