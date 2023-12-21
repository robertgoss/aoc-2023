use std::collections::{HashMap, HashSet};

pub struct Map {
    rocks: HashSet<(i64, i64)>,
    height: i64,
    width: i64,
    start: (i64, i64),
}

impl Map {
    pub fn from_grid(grid: &HashMap<(i64, i64), char>) -> Map {
        let rocks = grid
            .iter()
            .filter(|(_, ch)| **ch == '#')
            .map(|(id, _)| *id)
            .collect();
        let start = grid
            .iter()
            .filter(|(_, ch)| **ch == 'S')
            .map(|(id, _)| *id)
            .next()
            .unwrap();
        let height = grid.iter().map(|((i, _), _)| i + 1).max().unwrap_or(0);
        let width = grid.iter().map(|((_, j), _)| j + 1).max().unwrap_or(0);
        Map {
            rocks,
            start,
            height,
            width,
        }
    }

    pub fn steps(&self, num: usize) -> usize {
        let mut state = HashSet::new();
        state.insert(self.start);
        for _ in 0..num {
            state = self.next(&state);
        }
        state.len()
    }

    fn next(&self, initial: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
        let mut next_state = HashSet::new();
        for (i, j) in initial.iter() {
            if self.possible(i + 1, *j) {
                next_state.insert((i + 1, *j));
            }
            if self.possible(i - 1, *j) {
                next_state.insert((i - 1, *j));
            }
            if self.possible(*i, j + 1) {
                next_state.insert((*i, j + 1));
            }
            if self.possible(*i, j - 1) {
                next_state.insert((*i, j - 1));
            }
        }
        next_state
    }

    fn possible(&self, i: i64, j: i64) -> bool {
        if i < 0 || i >= self.height {
            return false;
        }
        if j < 0 || j >= self.width {
            return false;
        }
        !self.rocks.contains(&(i, j))
    }
}
