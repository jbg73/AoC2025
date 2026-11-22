use crate::map::{CellStatus, Map};

pub struct BeamManager {
    map: Map,
    current_idx: usize,
    splits: usize,
}

impl BeamManager {
    pub fn new(map: Map) -> Self {
        BeamManager {
            map,
            current_idx: 0,
            splits: 0,
        }
    }

    pub fn initialize(&mut self) {
        let mut start_row = 0;
        let mut start_col = 0;
        for (row_idx, row) in self.map.cells.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                if *value == CellStatus::Start {
                    start_row = row_idx;
                    start_col = col_idx;
                }
            }
        }

        self.map.cells[start_row + 1][start_col] = CellStatus::Beam;
    }

    pub fn run(&mut self) {
        while self.current_idx < self.map.cells.len() - 1 {
            self.step();
        }
    }

    pub fn result(&self) -> usize {
        self.splits
    }

    fn step(&mut self) {
        let current_row = &self.map.cells[self.current_idx];

        let beam_indexes = current_row
            .iter()
            .enumerate()
            .filter(|(col, cell)| **cell == CellStatus::Beam)
            .map(|(col, _)| col)
            .collect::<Vec<usize>>();

        let next_row = self.current_idx + 1;
        beam_indexes
            .iter()
            .for_each(|&col| match self.map.cells[next_row][col] {
                CellStatus::Start => todo!(),
                CellStatus::Empty => self.map.cells[next_row][col] = CellStatus::Beam,
                CellStatus::Splitter => self.spawn_beams(next_row, col),
                CellStatus::Beam => {}
            });

        self.current_idx += 1;
        // self.map.display();
    }

    fn spawn_beams(&mut self, row: usize, col: usize) {
        let mut spawned = false;
        // check left spawn
        if let CellStatus::Empty = self.map.cells[row][col - 1] {
            self.map.cells[row][col - 1] = CellStatus::Beam;
            spawned = true;
        }

        // check right spawn
        if let CellStatus::Empty = self.map.cells[row][col + 1] {
            self.map.cells[row][col + 1] = CellStatus::Beam;
            spawned = true;
        }

        if spawned {
            self.splits += 1;
        }
    }
}
