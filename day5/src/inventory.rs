use rayon::prelude::*;

type IngredientId = u64;

#[derive(Debug)]
pub struct Range(IngredientId, IngredientId);

impl From<&str> for Range {
    // todo: safer approach? What if the str is corrupted, how to handle errors?
    fn from(value: &str) -> Self {
        let range_bounds: Vec<_> = value.split('-').collect();
        let range_low = range_bounds[0].parse::<IngredientId>().unwrap();
        let range_high = range_bounds[1].parse::<IngredientId>().unwrap();

        Self(range_low, range_high)
    }
}

#[derive(Debug)]
pub struct Ingredient {
    id: IngredientId,
    fresh: bool,
    hits: usize,
}

impl From<&str> for Ingredient {
    fn from(value: &str) -> Self {
        let id = value.parse::<IngredientId>().unwrap();

        Ingredient {
            id,
            fresh: false,
            hits: 0,
        }
    }
}

pub struct InventoryManager {
    fresh_ranges: Vec<Range>,
    available_ingredients: Vec<Ingredient>,
}

impl InventoryManager {
    pub fn new(fresh_ranges: Vec<Range>, available_ingredients: Vec<Ingredient>) -> Self {
        Self {
            fresh_ranges,
            available_ingredients,
        }
    }

    pub fn get_fresh_ingredients_count(&self) -> usize {
        self.available_ingredients
            .par_iter()
            .filter(|&ingredient| ingredient.fresh)
            .count()
    }

    pub fn update_fresh_ingredients(&mut self) {
        self.available_ingredients
            .par_iter_mut()
            .for_each(|ingredient| {
                self.fresh_ranges.iter().for_each(|range| {
                    if ingredient.id >= range.0 && ingredient.id <= range.1 {
                        ingredient.fresh = true;
                    }
                })
            });
    }
}
