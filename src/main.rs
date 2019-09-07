use six_nimmt::ai::ai::AI;
use six_nimmt::ai::cli_player::CLIPlayer;
use six_nimmt::ai::random_ai::RandomAI;
use six_nimmt::ui::game::GameManager;

fn main() {
    let mut gm = GameManager::new();
    let players: Vec<Box<dyn AI>> = vec![
        Box::new(CLIPlayer::new("Alpha".to_string())),
        Box::new(RandomAI::new("Bravo".to_string(), 2)),
        Box::new(RandomAI::new("Charlie".to_string(), 3)),
        Box::new(RandomAI::new("Delta".to_string(), 4)),
        Box::new(RandomAI::new("Echo".to_string(), 5)),
        Box::new(RandomAI::new("Foxtrot".to_string(), 6)),
    ];
    let cards = (1..=104).collect();
    gm.initialize(28, players, cards);
    gm.run();
}
