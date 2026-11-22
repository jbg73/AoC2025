mod beam_manager;
mod map;

use beam_manager::BeamManager;
use map::Map;

fn main() {
    let content = std::fs::read_to_string("day7/data/input.txt").expect("Error opening file");
    let map = Map::from(content);

    let mut beam_manager = BeamManager::new(map);
    beam_manager.initialize();
    beam_manager.run();

    println!("Result 7.1: {}", beam_manager.result());
}
