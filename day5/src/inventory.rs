use std::{
    cmp::{max, min},
    collections::HashSet,
};

use rayon::{prelude::*, vec};

type IngredientId = u64;

#[derive(Debug, Clone, Copy, Ord, PartialEq, PartialOrd, Eq)]
pub struct Range {
    start: IngredientId,
    end: IngredientId,
    merging: bool,
}

impl Range {
    fn new(start: IngredientId, end: IngredientId, merging: bool) -> Self {
        Self {
            start,
            end,
            merging,
        }
    }
    fn merge(left_range: &Self, right_range: &Self) -> Self {
        Self::new(
            min(left_range.start, right_range.start),
            max(left_range.end, right_range.end),
            false,
        )
    }

    fn is_mergeable(left_range: &Self, right_range: &Self) -> bool {
        left_range.end >= right_range.start
    }
}

impl From<&str> for Range {
    // todo: safer approach? What if the str is corrupted, how to handle errors?
    fn from(value: &str) -> Self {
        let range_bounds: Vec<_> = value.split('-').collect();
        let range_low = range_bounds[0].parse::<IngredientId>().unwrap();
        let range_high = range_bounds[1].parse::<IngredientId>().unwrap();

        Self {
            start: range_low,
            end: range_high,
            merging: false,
        }
    }
}

#[derive(Debug)]
pub struct Ingredient {
    id: IngredientId,
    fresh: bool,
}

impl From<&str> for Ingredient {
    fn from(value: &str) -> Self {
        let id = value.parse::<IngredientId>().unwrap();

        Ingredient { id, fresh: false }
    }
}

pub struct InventoryManager {
    fresh_ranges: Vec<Range>,
    available_ingredients: Vec<Ingredient>,
}

impl InventoryManager {
    pub fn new(fresh_ranges: Vec<Range>, available_ingredients: Vec<Ingredient>) -> Self {
        let mut inventory_manager = Self {
            fresh_ranges,
            available_ingredients,
        };

        inventory_manager.fresh_ranges.sort();

        inventory_manager
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
                    if ingredient.id >= range.start && ingredient.id <= range.end {
                        ingredient.fresh = true;
                    }
                })
            });
    }

    fn get_mergeable_chunk(
        ranges: &[Range],
        current_position: usize,
    ) -> Option<(Vec<usize>, usize)> {
        let mut mergeable: Vec<usize> = vec![];
        let mut next_position = current_position;
        let mut found_first = false;

        for i in current_position..(ranges.len() - 1) {
            if Range::is_mergeable(&ranges[i], &ranges[i + 1]) {
                if !mergeable.contains(&i) {
                    mergeable.push(i);
                }
                if !mergeable.contains(&(i + 1)) {
                    mergeable.push(i + 1);
                }
                if !found_first {
                    next_position = i;
                    found_first = true;
                }
            } else if !mergeable.is_empty() {
                break;
            }
        }

        if !mergeable.is_empty() {
            return Some((mergeable, next_position));
        }

        None
    }

    fn merge(&mut self, mergeable_indexes: &Vec<usize>, current_position: usize) {
        let left_range = self.fresh_ranges[mergeable_indexes.first().unwrap().to_owned()];
        let right_range = self.fresh_ranges[mergeable_indexes.last().unwrap().to_owned()];
        let merged_range = Range::merge(&left_range, &right_range);

        mergeable_indexes.iter().rev().for_each(|&rm_idx| {
            self.fresh_ranges.remove(rm_idx);
        });

        self.fresh_ranges.insert(current_position, merged_range);
    }

    pub fn get_possible_fresh_ingredients(&mut self) -> u64 {
        let mut current_position = 0;
        while let Some((mergeable_chunk, next_position)) =
            InventoryManager::get_mergeable_chunk(&self.fresh_ranges, current_position)
        {
            self.merge(&mergeable_chunk, next_position);
            current_position = next_position;
        }

        let mut all_fresh: u64 = 0;
        self.fresh_ranges.iter().for_each(|range| {
            all_fresh += (range.end - range.start + 1) as u64;
        });
        all_fresh
    }
}
