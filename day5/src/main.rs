mod inventory;
mod parser;

use parser::InputParser;

use crate::inventory::InventoryManager;

fn main() {
    let (ingredient_ranges, available_ingredients) = InputParser::read_data("day5/data/input.txt");

    let mut inventory = InventoryManager::new(ingredient_ranges, available_ingredients);
    inventory.update_fresh_ingredients();

    let fresh_ingredients_count = inventory.get_fresh_ingredients_count();
    println!("Total fresh: {}", fresh_ingredients_count);

    let all_fresh_ingredients = inventory.get_possible_fresh_ingredients();
    println!("Total possible fresh: {}", all_fresh_ingredients);
    // 6524784925797
    // 350513176552974
    // ^[0-9]{13}-
    //-[0-9]{13}$
    //350513176552950
}
