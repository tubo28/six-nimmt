use six_nimmt::ui::game::GameManager;
use six_nimmt::ui::random_ai::RandomAI;
use six_nimmt::ui::cli_player::CLIPlayer;

use six_nimmt::ui::ai::AI;

fn main() {
    println!("Hello, world!");

    CLIPlayer::new("aoaa".to_string(), 0);

    
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
