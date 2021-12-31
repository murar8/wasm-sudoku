use std::{
    collections::{HashMap, HashSet},
    iter::{empty, FromIterator},
    num::NonZeroU8,
};

use rand::{prelude::SliceRandom, rngs::StdRng, Rng, SeedableRng};
use wasm_bindgen::prelude::*;

const STD_GRID_SPAN: usize = 9;
const STD_BLOCK_SPAN: usize = 3;
const STD_GRID_AREA: usize = STD_GRID_SPAN * STD_GRID_SPAN;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SolveResult {
    Solved,
    Unsolvable,
}

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum PlayResult {
    Ok,
    InvalidMove,
    InvalidInput,
}

#[derive(Clone, Copy, PartialEq)]
enum SolveStrategy {
    Random,
    Ordered,
}

#[wasm_bindgen]
pub struct Sudoku {
    pub grid_span: usize,
    pub block_span: usize,
    grid: Vec<Option<NonZeroU8>>,
    initial_cells: HashSet<usize>,
    rng: StdRng,
}

#[wasm_bindgen]
impl Sudoku {
    fn get_row_entries(&self, i: usize) -> HashMap<usize, NonZeroU8> {
        let start = i - i % self.grid_span;

        self.grid[start..start + self.grid_span]
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.map(|v| (i + start, v)))
            .collect()
    }

    fn get_column_entries(&self, i: usize) -> HashMap<usize, NonZeroU8> {
        let start = i % self.grid_span;

        (start..self.grid_span * self.grid_span)
            .step_by(self.grid_span)
            .filter_map(|i| self.grid[i].map(|v| (i, v)))
            .collect()
    }

    fn get_block_entries(&self, i: usize) -> HashMap<usize, NonZeroU8> {
        let x = i % self.grid_span;
        let y = i / self.grid_span;
        let x0 = x - x % self.block_span;
        let y0 = y - y % self.block_span;

        let starts = (0..self.block_span).map(move |row| (y0 + row) * self.grid_span + x0);
        let indexes = starts.flat_map(|i| i..i + self.block_span);

        indexes
            .filter_map(|i| self.grid[i].map(|v| (i, v)))
            .collect()
    }

    fn get_intersecting_values(&self, i: usize) -> HashSet<NonZeroU8> {
        empty()
            .chain(self.get_row_entries(i).into_values())
            .chain(self.get_column_entries(i).into_values())
            .chain(self.get_block_entries(i).into_values())
            .collect()
    }

    fn get_possible_values(&self, i: usize) -> Vec<NonZeroU8> {
        let intersectors = self.get_intersecting_values(i);

        (1..=9)
            .filter_map(NonZeroU8::new)
            .filter(move |v| !intersectors.contains(v))
            .collect()
    }

    fn fill(&mut self, i: usize, strategy: SolveStrategy) -> SolveResult {
        if i >= self.grid.len() {
            return SolveResult::Solved;
        }

        if self.grid[i].is_some() {
            return self.fill(i + 1, strategy);
        }

        let mut possible_values = self.get_possible_values(i);

        if strategy == SolveStrategy::Random {
            possible_values.shuffle(&mut self.rng);
        }

        for c in possible_values {
            self.grid[i] = Some(c);

            if self.fill(i + 1, strategy) == SolveResult::Solved {
                return SolveResult::Solved;
            }
        }

        self.grid[i] = None;

        SolveResult::Unsolvable
    }

    pub fn solve(&mut self) -> SolveResult {
        self.fill(0, SolveStrategy::Ordered)
    }

    fn is_valid_move(&self, i: usize, value: u8) -> bool {
        if self.initial_cells.contains(&i) {
            return false;
        }

        if value == 0 {
            return true;
        }

        !self
            .get_intersecting_values(i)
            .iter()
            .any(|entry| entry.get() == value)
    }

    pub fn play(&mut self, i: usize, value: u8) -> PlayResult {
        if i >= self.grid_span * self.grid_span || value > 9 {
            return PlayResult::InvalidInput;
        }

        if !self.is_valid_move(i, value) {
            return PlayResult::InvalidMove;
        }

        self.grid[i] = match value {
            0 => None,
            _ => NonZeroU8::new(value),
        };

        PlayResult::Ok
    }

    pub fn reset(&mut self) {
        let deletables = HashSet::from_iter(0..self.grid.len());
        let deletables = deletables.difference(&self.initial_cells);

        for &i in deletables {
            self.grid[i] = None;
        }
    }

    pub fn random(seed: u64, clues: usize) -> Self {
        if clues > STD_GRID_AREA {
            panic!("Number of clues is higher than the grid size.")
        }

        let mut sudoku = Sudoku {
            grid_span: STD_GRID_SPAN,
            block_span: STD_BLOCK_SPAN,
            grid: vec![None; STD_GRID_AREA],
            initial_cells: HashSet::with_capacity(clues),
            rng: StdRng::seed_from_u64(seed),
        };

        sudoku.fill(0, SolveStrategy::Random);

        while sudoku.initial_cells.len() < clues {
            sudoku
                .initial_cells
                .insert((sudoku.rng.gen::<f32>() * (sudoku.grid_span as f32).powi(2)) as usize);
        }

        sudoku.reset();

        sudoku
    }

    #[wasm_bindgen(js_name = getGrid)]
    pub fn get_grid(&self) -> *const Option<NonZeroU8> {
        self.grid.as_ptr()
    }

    #[wasm_bindgen(js_name = isMutableCell)]
    pub fn is_mutable_cell(&self, i: usize) -> bool {
        !self.initial_cells.contains(&i)
    }

    #[wasm_bindgen(js_name = isSolved)]
    pub fn is_solved(&self) -> bool {
        !self.grid.iter().any(|v| v.is_none())
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU8;

    use rand::{prelude::StdRng, SeedableRng};

    use super::*;

    const TEST_VALUES: [u8; 81] = [
        0, 0, 0, 0, 0, 0, 0, 0, 7, // 0
        7, 6, 3, 0, 0, 0, 9, 0, 1, // 1
        1, 0, 5, 3, 6, 7, 2, 4, 8, // 2
        0, 0, 1, 8, 3, 0, 4, 7, 0, // 3
        9, 0, 0, 1, 7, 6, 0, 8, 3, // 4
        3, 0, 0, 0, 0, 4, 0, 0, 0, // 5
        0, 8, 9, 7, 0, 0, 0, 2, 5, // 6
        5, 3, 0, 0, 8, 0, 0, 1, 9, // 7
        0, 1, 7, 6, 0, 0, 8, 0, 0, // 8
    ];

    const IMPOSSIBLE_VALUES: [u8; 81] = [
        2, 5, 6, 0, 4, 9, 8, 3, 7, // 0
        1, 8, 3, 5, 0, 7, 9, 6, 4, // 1
        9, 7, 4, 3, 8, 6, 2, 5, 1, // 2
        8, 4, 9, 1, 6, 2, 3, 7, 5, // 3
        5, 6, 2, 7, 9, 3, 4, 1, 8, // 4
        7, 3, 1, 4, 5, 8, 6, 2, 9, // 5
        6, 9, 7, 8, 3, 1, 5, 4, 2, // 6
        4, 2, 8, 6, 7, 5, 1, 9, 3, // 7
        3, 1, 5, 9, 2, 4, 7, 8, 6, // 8
    ];

    fn create_sudoku(values: [u8; 81]) -> Sudoku {
        let grid = values
            .iter()
            .map(|&v| (NonZeroU8::new(v)))
            .collect::<Vec<_>>();

        let initial_cells = values
            .iter()
            .enumerate()
            .filter(|(_, &v)| v != 0)
            .map(|(i, _)| i)
            .collect();

        Sudoku {
            grid_span: 9,
            block_span: 3,
            rng: StdRng::seed_from_u64(42),
            grid,
            initial_cells,
        }
    }

    macro_rules! nonzerou8_map {
        ($( $i:expr => $v:expr ), *) => {
            std::collections::HashMap::from([
                    $(
                        ($i, std::num::NonZeroU8::new($v).unwrap()),
                    )*
                ]
            )
        };
    }

    macro_rules! nonzerou8_set {
        ($( $v:expr ), *) => {
            std::collections::HashSet::from([
                    $( std::num::NonZeroU8::new($v).unwrap(), )*
                ]
            )
        };
    }

    macro_rules! nonzerou8_vec {
        ($( $v:expr ), *) => {
            std::vec::Vec::from([
                    $( std::num::NonZeroU8::new($v).unwrap(), )*
                ]
            )
        };
    }

    #[test]
    fn test_get_row_entries() {
        let sudoku = create_sudoku(TEST_VALUES);

        assert_eq!(sudoku.get_row_entries(0), nonzerou8_map!(8 => 7));
        assert_eq!(
            sudoku.get_row_entries(40),
            nonzerou8_map!(36 => 9, 39 => 1, 40 => 7, 41 => 6, 43 => 8, 44 => 3)
        );
        assert_eq!(
            sudoku.get_row_entries(80),
            nonzerou8_map!(73 => 1, 74 => 7, 75 => 6, 78 => 8)
        );
    }

    #[test]
    fn test_get_column_entries() {
        let sudoku = create_sudoku(TEST_VALUES);

        assert_eq!(
            sudoku.get_column_entries(0),
            nonzerou8_map!(9 => 7, 18 => 1, 36 => 9, 45 => 3, 63 => 5)
        );
        assert_eq!(
            sudoku.get_column_entries(40),
            nonzerou8_map!(22 => 6, 31 => 3, 40 => 7, 67 => 8)
        );
        assert_eq!(
            sudoku.get_column_entries(80),
            nonzerou8_map!(8 => 7, 17 => 1, 26 => 8, 44 => 3, 62 => 5, 71 => 9)
        );
    }

    #[test]
    fn test_get_block_entries() {
        let sudoku = create_sudoku(TEST_VALUES);

        assert_eq!(
            sudoku.get_block_entries(40),
            nonzerou8_map!(30 => 8, 31 => 3, 39 => 1, 40 => 7, 41 => 6, 50 => 4)
        );
    }

    #[test]
    fn test_get_intersecting_values() {
        let sudoku = create_sudoku(TEST_VALUES);

        assert_eq!(
            sudoku.get_intersecting_values(0),
            nonzerou8_set!(1, 3, 5, 6, 7, 9)
        );
        assert_eq!(
            sudoku.get_intersecting_values(40),
            nonzerou8_set!(1, 3, 4, 6, 7, 8, 9)
        );
        assert_eq!(
            sudoku.get_intersecting_values(80),
            nonzerou8_set!(1, 2, 3, 5, 6, 7, 8, 9)
        );
    }

    #[test]
    fn test_get_possible_values() {
        let sudoku = create_sudoku(TEST_VALUES);
        assert_eq!(sudoku.get_possible_values(0), nonzerou8_vec!(2, 4, 8));
        assert_eq!(sudoku.get_possible_values(40), nonzerou8_vec!(2, 5));
        assert_eq!(sudoku.get_possible_values(80), nonzerou8_vec!(4));
    }

    #[test]
    fn test_solve() {
        let mut sudoku = create_sudoku(TEST_VALUES);
        assert_eq!(sudoku.solve(), SolveResult::Solved);

        let mut impossible = create_sudoku(IMPOSSIBLE_VALUES);
        assert_eq!(impossible.solve(), SolveResult::Unsolvable);
    }

    #[test]
    fn test_is_valid_move() {
        let sudoku = create_sudoku(TEST_VALUES);

        assert!(sudoku.is_valid_move(0, 0));
        assert!(!sudoku.is_valid_move(9, 0));
        assert!(!sudoku.is_valid_move(40, 4));
        assert!(!sudoku.is_valid_move(80, 6));
        assert!(!sudoku.is_valid_move(80, 3));
    }

    #[test]
    fn test_play() {
        let mut sudoku = create_sudoku(TEST_VALUES);

        assert_eq!(sudoku.play(0, 10), PlayResult::InvalidInput);
        assert_eq!(sudoku.play(100, 0), PlayResult::InvalidInput);
        assert_eq!(sudoku.play(0, 7), PlayResult::InvalidMove);
        assert_eq!(sudoku.play(0, 4), PlayResult::Ok);
        assert_eq!(sudoku.grid[0].unwrap().get(), 4);
        assert_eq!(sudoku.play(0, 0), PlayResult::Ok);
        assert_eq!(sudoku.grid[0], None);
    }

    #[test]
    fn test_reset() {
        let mut sudoku = create_sudoku(TEST_VALUES);
        sudoku.grid[0] = NonZeroU8::new(4);
        sudoku.grid[80] = NonZeroU8::new(4);
        sudoku.reset();

        assert_eq!(sudoku.grid[0], None);
        assert_eq!(sudoku.grid[80], None);
    }

    #[test]
    fn test_random() {
        let mut sudoku0 = Sudoku::random(42, 10);
        let sudoku1 = Sudoku::random(42, 10);
        let sudoku2 = Sudoku::random(100, 10);

        assert_eq!(sudoku0.grid, sudoku1.grid);
        assert_ne!(sudoku0.grid, sudoku2.grid);
        assert_eq!(sudoku0.solve(), SolveResult::Solved);
        assert_eq!(sudoku0.initial_cells.len(), 10);
    }

    #[test]
    #[should_panic]
    fn test_random_clues() {
        Sudoku::random(42, 100);
    }

    #[test]
    fn test_is_mutable_cell() {
        let sudoku = create_sudoku(TEST_VALUES);
        assert!(sudoku.is_mutable_cell(0));
        assert!(!sudoku.is_mutable_cell(40));
    }

    #[test]
    fn test_is_solved() {
        let mut sudoku = create_sudoku(TEST_VALUES);
        assert!(!sudoku.is_solved());
        sudoku.solve();
        assert!(sudoku.is_solved());
    }
}
