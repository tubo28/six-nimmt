use six_nimmt::ui::cli_player::CLIPlayer;
use six_nimmt::ui::game::GameManager;
use six_nimmt::ui::random_ai::RandomAI;

use six_nimmt::ui::ai::AI;

fn main() {
    let seed = 32;
    let mut gm = GameManager::new();
    let players: Vec<Box<AI>> = vec![
        Box::new(CLIPlayer::new("Alpha".to_string())),
        Box::new(RandomAI::new("Bravo".to_string(), seed)),
        Box::new(RandomAI::new("Charlie".to_string(), seed)),
        Box::new(RandomAI::new("Delta".to_string(), seed)),
        Box::new(RandomAI::new("Echo".to_string(), seed)),
        Box::new(RandomAI::new("Foxtrot".to_string(), seed)),
    ];
    let cards = (1..=104).collect();
    gm.initialize(seed, players, cards);
    gm.run();
}
