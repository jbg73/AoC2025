#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    Occupied,
    Empty,
}

impl From<&char> for CellState {
    fn from(value: &char) -> Self {
        match value {
            '.' => CellState::Empty,
            '@' => CellState::Occupied,
            _ => panic!("Invalid character while reading the map"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position(pub usize, pub usize);

#[derive(Debug)]
pub struct Cell<T> {
    position: Position,
    state: T,
}

impl<T> Cell<T> {
    pub fn new(position: Position, state: T) -> Self {
        Self { position, state }
    }
}

pub type Row = Vec<Cell<CellState>>;
pub type Matrix = Vec<Row>;

#[derive(Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub struct FactoryMap {
    grid: Matrix,
    rows: usize,
    cols: usize,
    relevant_directions: Vec<Direction>,
}

impl FactoryMap {
    pub fn new(grid: Matrix) -> Self {
        let rows = grid.len();
        let cols = grid[0].len();
        Self {
            grid,
            rows,
            cols,
            relevant_directions: vec![
                Direction::Up,
                Direction::UpRight,
                Direction::Right,
                Direction::DownRight,
                Direction::Down,
                Direction::DownLeft,
                Direction::Left,
                Direction::UpLeft,
            ],
        }
    }

    fn get_next_position(current_position: &Position, direction: &Direction) -> Position {
        let (x, y) = (current_position.0, current_position.1);
        match direction {
            Direction::Up => Position(x - 1, y),
            Direction::UpRight => Position(x - 1, y + 1),
            Direction::Right => Position(x, y + 1),
            Direction::DownRight => Position(x + 1, y + 1),
            Direction::Down => Position(x + 1, y),
            Direction::DownLeft => Position(x + 1, y - 1),
            Direction::Left => Position(x, y - 1),
            Direction::UpLeft => Position(x - 1, y - 1),
        }
    }

    fn get_state_in_position(&self, position: &Position) -> CellState {
        self.grid[position.0][position.1].state
    }

    fn bound_check_in_direction(&self, current_position: &Position, direction: &Direction) -> bool {
        let (x, y) = (current_position.0, current_position.1);
        match direction {
            Direction::Up => x > 0,
            Direction::UpRight => x > 0 && y < self.cols - 1,
            Direction::Right => y < self.cols - 1,
            Direction::DownRight => x < self.rows - 1 && y < self.cols - 1,
            Direction::Down => x < self.rows - 1,
            Direction::DownLeft => x < self.rows - 1 && y > 0,
            Direction::Left => y > 0,
            Direction::UpLeft => x > 0 && y > 0,
        }
    }

    fn check_adjacent_paper_in_direction(
        &self,
        current_position: &Position,
        direction: &Direction,
    ) -> bool {
        if !self.bound_check_in_direction(current_position, direction) {
            return false;
        }
        let target_position = FactoryMap::get_next_position(current_position, direction);

        self.get_state_in_position(&target_position) == CellState::Occupied
    }

    fn count_adjacent_papers(&self, current_position: &Position) -> i32 {
        let mut adjacent_papers = 0;

        self.relevant_directions.iter().for_each(|dir| {
            if self.check_adjacent_paper_in_direction(current_position, dir) {
                adjacent_papers += 1
            }
        });

        adjacent_papers
    }

    pub fn count_all_accessible_papers(&self) -> Option<Vec<Position>> {
        let mut accessible_papers: Vec<Position> = vec![];

        self.grid.iter().for_each(|row| {
            row.iter().for_each(|c| {
                if self.get_state_in_position(&c.position) == CellState::Occupied
                    && self.count_adjacent_papers(&c.position) < 4
                {
                    accessible_papers.push(c.position);
                }
            })
        });

        if accessible_papers.is_empty() {
            return None;
        }

        Some(accessible_papers)
    }

    pub fn update_grid(&mut self, removed_papers: Vec<Position>) {
        removed_papers
            .iter()
            .for_each(|p| self.grid[p.0][p.1].state = CellState::Empty);
    }
}
