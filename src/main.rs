use six_nimmt::ui::game;
use six_nimmt::ui::random_ai;

fn main() {
    use game::GameManager;
    use random_ai::RandomAI;

    println!("Hello, world!");
    let seed = 32;
    let mut gm = GameManager::new();
    let players = vec![
        RandomAI::new("Alpha".to_string(), seed),
        RandomAI::new("Bravo".to_string(), seed),
        RandomAI::new("Charlie".to_string(), seed),
        RandomAI::new("Delta".to_string(), seed),
        RandomAI::new("Echo".to_string(), seed),
        RandomAI::new("Foxtrot".to_string(), seed),
    ];
    let cards = (1..=104).collect();
    gm.initialize(seed, &players[..], &cards);
    gm.run();
}
