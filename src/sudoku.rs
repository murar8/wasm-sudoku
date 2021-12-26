use std::{
    collections::{BTreeSet, HashSet},
    iter::empty,
};

use either::Either;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

pub const GRID_SIZE: usize = 9;
pub const BLOCK_SIZE: usize = 3;

const GRID_AREA: usize = GRID_SIZE * GRID_SIZE;
const CLUE_QTY: usize = 21;

#[wasm_bindgen]
pub struct Sudoku {
    grid: [u8; GRID_AREA],
}

#[wasm_bindgen]
impl Sudoku {
    fn get_row(&self, y: usize) -> impl Iterator<Item = u8> + '_ {
        self.grid[GRID_SIZE * y..GRID_SIZE * (y + 1)]
            .iter()
            .copied()
    }

    fn get_column(&self, x: usize) -> impl Iterator<Item = u8> + '_ {
        (0..GRID_AREA)
            .step_by(GRID_SIZE)
            .map(move |i| self.grid[i + x])
    }

    fn get_block(&self, x: usize, y: usize) -> impl Iterator<Item = u8> + '_ {
        let x0 = x - x % BLOCK_SIZE;
        let y0 = y - y % BLOCK_SIZE;

        (0..BLOCK_SIZE)
            .map(move |row| (y0 + row) * GRID_SIZE + x0)
            .flat_map(move |ys| &self.grid[ys..ys + BLOCK_SIZE])
            .copied()
    }

    fn get_intersections(&self, x: usize, y: usize) -> HashSet<u8> {
        empty()
            .chain(self.get_block(x, y))
            .chain(self.get_row(y))
            .chain(self.get_column(x))
            .collect()
    }

    fn solve_internal(&mut self, i: usize, random_order: bool) -> bool {
        if i >= self.grid.len() {
            return true;
        }

        if self.grid[i] != 0 {
            return self.solve_internal(i + 1, random_order);
        }

        let intersections = self.get_intersections(i % GRID_SIZE, i / GRID_SIZE);

        let candidates = (1..=9).collect::<HashSet<u8>>();
        let candidates = candidates.difference(&intersections);

        let candidates = if random_order {
            Either::Left(candidates.collect::<HashSet<_>>().into_iter())
        } else {
            Either::Right(candidates.collect::<BTreeSet<_>>().into_iter())
        };

        for &c in candidates {
            self.grid[i] = c;

            if self.solve_internal(i + 1, random_order) {
                return true;
            }
        }

        self.grid[i] = 0;

        false
    }

    pub fn solve(&mut self) -> bool {
        self.solve_internal(0, false)
    }

    pub fn play(&mut self, x: usize, y: usize, value: u8) -> bool {
        if x >= GRID_SIZE
            || y >= GRID_SIZE
            || value < 1
            || value > 9
            || self.get_intersections(x, y).contains(&value)
        {
            false
        } else {
            self.grid[x + y * GRID_SIZE] = value;
            true
        }
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let mut empty_cells = HashSet::<usize>::with_capacity(GRID_AREA - CLUE_QTY);
        let mut sudoku = Sudoku {
            grid: [0; GRID_AREA],
        };

        sudoku.solve_internal(0, true);

        while empty_cells.len() < GRID_AREA - CLUE_QTY {
            empty_cells.insert((rng.gen::<f32>() * GRID_AREA as f32) as usize);
        }

        for i in empty_cells {
            sudoku.grid[i] = 0;
        }

        sudoku
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::sudoku::CLUE_QTY;

    use super::{Sudoku, GRID_AREA, GRID_SIZE};

    const TEST_GRID: [u8; GRID_AREA] = [
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

    #[test]
    fn test_get_row() {
        let sudoku = Sudoku { grid: TEST_GRID };

        assert_eq!(
            sudoku.get_row(0).collect::<Vec<_>>(),
            [0, 0, 0, 0, 0, 0, 0, 0, 7]
        );

        assert_eq!(
            sudoku.get_row(2).collect::<Vec<_>>(),
            [1, 0, 5, 3, 6, 7, 2, 4, 8]
        );

        assert_eq!(
            sudoku.get_row(GRID_SIZE - 1).collect::<Vec<_>>(),
            [0, 1, 7, 6, 0, 0, 8, 0, 0]
        );
    }

    #[test]
    fn test_get_column() {
        let sudoku = Sudoku { grid: TEST_GRID };

        assert_eq!(
            sudoku.get_column(0).collect::<Vec<_>>(),
            vec![0, 7, 1, 0, 9, 3, 0, 5, 0]
        );

        assert_eq!(
            sudoku.get_column(2).collect::<Vec<_>>(),
            vec![0, 3, 5, 1, 0, 0, 9, 0, 7]
        );

        assert_eq!(
            sudoku.get_column(GRID_SIZE - 1).collect::<Vec<_>>(),
            vec![7, 1, 8, 0, 3, 0, 5, 9, 0]
        );
    }

    #[test]
    fn test_get_block() {
        let sudoku = Sudoku { grid: TEST_GRID };

        assert_eq!(
            sudoku.get_block(0, 0).collect::<Vec<_>>(),
            vec![0, 0, 0, 7, 6, 3, 1, 0, 5]
        );

        assert_eq!(
            sudoku.get_block(3, 0).collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 0, 3, 6, 7]
        );

        assert_eq!(
            sudoku.get_block(4, 4).collect::<Vec<_>>(),
            vec![8, 3, 0, 1, 7, 6, 0, 0, 4]
        );

        assert_eq!(
            sudoku
                .get_block(GRID_SIZE - 1, GRID_SIZE - 1)
                .collect::<Vec<_>>(),
            vec![0, 2, 5, 0, 1, 9, 8, 0, 0]
        );
    }

    #[test]
    fn test_get_intersections() {
        let sudoku = Sudoku { grid: TEST_GRID };

        assert_eq!(
            sudoku.get_intersections(0, 0),
            HashSet::from([0, 1, 3, 5, 6, 7, 9])
        );

        assert_eq!(
            sudoku.get_intersections(4, 4),
            HashSet::from([0, 1, 3, 4, 6, 7, 8, 9])
        );

        assert_eq!(
            sudoku.get_intersections(GRID_SIZE - 1, GRID_SIZE - 1),
            HashSet::from([0, 1, 2, 3, 5, 6, 7, 8, 9])
        );
    }

    #[test]
    fn test_play() {
        let mut sudoku = Sudoku { grid: TEST_GRID };

        assert!(sudoku.play(0, 0, 4));
        assert!(sudoku.play(GRID_SIZE - 1, GRID_SIZE - 1, 4));

        assert!(!sudoku.play(0, 0, 6));
        assert!(!sudoku.play(GRID_SIZE - 1, GRID_SIZE - 1, 3));
        assert!(!sudoku.play(GRID_SIZE - 1, GRID_SIZE - 1, 6));
        assert!(!sudoku.play(0, 0, 0));
        assert!(!sudoku.play(0, 0, 10));
        assert!(!sudoku.play(10, 0, 4));
        assert!(!sudoku.play(0, 10, 0));
    }

    #[test]
    fn test_solve() {
        let mut sudoku = Sudoku { grid: TEST_GRID };
        assert!(sudoku.solve());
        assert!(sudoku.grid.iter().all(|&v| v != 0));
    }

    #[test]
    fn test_random() {
        let mut sudoku = Sudoku::random();
        assert_eq!(sudoku.grid.iter().filter(|&&v| v != 0).count(), CLUE_QTY);
        assert!(sudoku.solve());
    }
}
