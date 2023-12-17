use std::collections::{HashMap, HashSet};

pub struct Beam {
    fixed: HashSet<(i64, i64)>,
    movable: HashSet<(i64, i64)>,
    height: i64,
    width: i64,
}

fn char_grid(grid: &HashMap<(i64, i64), char>, base: char) -> HashSet<(i64, i64)> {
    grid.iter()
        .filter(|(_, ch)| **ch == base)
        .map(|(id, _)| *id)
        .collect()
}

impl Beam {
    pub fn from_grid(grid: &HashMap<(i64, i64), char>) -> Beam {
        let fixed = char_grid(grid, '#');
        let movable = char_grid(grid, 'O');
        let height = grid.iter().map(|((i, _), _)| i + 1).max().unwrap_or(0);
        let width = grid.iter().map(|((_, j), _)| j + 1).max().unwrap_or(0);
        Beam {
            fixed,
            movable,
            height,
            width,
        }
    }

    pub fn score_north(&mut self) -> usize {
        self.north();
        self.score()
    }

    fn spin(&mut self) {
        self.north();
        self.west();
        self.south();
        self.east()
    }

    pub fn score_spin(&mut self, final_index: usize) -> usize {
        let mut prev: HashMap<Vec<(i64, i64)>, usize> = HashMap::new();
        for i in 0.. {
            let vals: Vec<(i64, i64)> = self.movable.iter().cloned().collect();
            if let Some(prev_i) = prev.get(&vals) {
                let repeat = i - prev_i;
                let rem = (final_index - i) % repeat;
                for _ in 0..rem {
                    self.spin();
                }
                break;
            }
            prev.insert(vals, i);
            self.spin();
        }
        self.score()
    }

    fn north(&mut self) {
        loop {
            let to_move: Vec<(i64, i64)> = self
                .movable
                .iter()
                .filter(|(i, j)| *i > 0 && !self.contains(*i - 1, *j))
                .cloned()
                .collect();
            if to_move.is_empty() {
                break;
            }
            for (i, j) in to_move {
                self.movable.remove(&(i, j));
                self.movable.insert((i - 1, j));
            }
        }
    }

    fn west(&mut self) {
        loop {
            let to_move: Vec<(i64, i64)> = self
                .movable
                .iter()
                .filter(|(i, j)| *j > 0 && !self.contains(*i, *j - 1))
                .cloned()
                .collect();
            if to_move.is_empty() {
                break;
            }
            for (i, j) in to_move {
                self.movable.remove(&(i, j));
                self.movable.insert((i, j - 1));
            }
        }
    }

    fn south(&mut self) {
        loop {
            let to_move: Vec<(i64, i64)> = self
                .movable
                .iter()
                .filter(|(i, j)| *i < (self.height - 1) && !self.contains(*i + 1, *j))
                .cloned()
                .collect();
            if to_move.is_empty() {
                break;
            }
            for (i, j) in to_move {
                self.movable.remove(&(i, j));
                self.movable.insert((i + 1, j));
            }
        }
    }

    fn east(&mut self) {
        loop {
            let to_move: Vec<(i64, i64)> = self
                .movable
                .iter()
                .filter(|(i, j)| *j < (self.width - 1) && !self.contains(*i, *j + 1))
                .cloned()
                .collect();
            if to_move.is_empty() {
                break;
            }
            for (i, j) in to_move {
                self.movable.remove(&(i, j));
                self.movable.insert((i, j + 1));
            }
        }
    }

    fn contains(&self, i: i64, j: i64) -> bool {
        self.fixed.contains(&(i, j)) || self.movable.contains(&(i, j))
    }

    fn score(&self) -> usize {
        self.movable
            .iter()
            .map(|(i, _)| (self.height - i) as usize)
            .sum()
    }
}
