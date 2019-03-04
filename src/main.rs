mod ui;

fn main() {
    println!("Hello, world!");
    let seed = unimplemented!();
    let gm = ui::GameManager::new(seed);
    gm.run();
}
