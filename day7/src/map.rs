#[derive(Copy, Clone, PartialEq)]
pub enum CellStatus {
    Start,
    Empty,
    Splitter,
    Beam,
}

pub struct Map {
    pub cells: Vec<Vec<CellStatus>>,
}

impl Map {
    fn new() -> Self {
        Map { cells: Vec::new() }
    }

    pub fn query(&self, x: usize, y: usize) -> CellStatus {
        self.cells[x][y]
    }

    pub fn display(&self) {
        self.cells.iter().for_each(|row| {
            row.iter().for_each(|cell| match cell {
                CellStatus::Start => print!("S"),
                CellStatus::Empty => print!("."),
                CellStatus::Splitter => print!("^"),
                CellStatus::Beam => print!("|"),
            });
            println!();
        });
        println!();
    }
}

impl From<String> for Map {
    fn from(content: String) -> Map {
        let mut map = Map::new();
        content.lines().for_each(|data| {
            let row = data
                .chars()
                .map(|value| match value {
                    'S' => CellStatus::Start,
                    '.' => CellStatus::Empty,
                    '^' => CellStatus::Splitter,
                    _ => panic!("Bad format cell in map"),
                })
                .collect::<Vec<CellStatus>>();
            map.cells.push(row);
        });

        map
    }
}
