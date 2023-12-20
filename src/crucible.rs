use std::cmp::min;
use std::collections::HashMap;

use super::utils::Dir;

pub struct City {
    blocks: HashMap<(i64, i64), u8>,
    width: i64,
    height: i64,
}

fn u8_from_char(ch: char) -> u8 {
    ch.to_digit(10).unwrap() as u8
}

impl City {
    pub fn from_grid(grid: &HashMap<(i64, i64), char>) -> City {
        let blocks: HashMap<(i64, i64), u8> = grid
            .iter()
            .map(|(ind, ch)| (*ind, u8_from_char(*ch)))
            .collect();
        let height = grid.iter().map(|((i, _), _)| i + 1).max().unwrap_or(0);
        let width = grid.iter().map(|((_, j), _)| j + 1).max().unwrap_or(0);
        City {
            blocks,
            width,
            height,
        }
    }

    pub fn least_path(&self, min_l: i64, max_l: i64) -> usize {
        let mut least_paths: HashMap<(i64, i64, Dir), usize> = HashMap::new();
        let end_i = self.height - 1;
        let end_j = self.width - 1;
        for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
            least_paths.insert((end_i, end_j, dir), 0);
        }
        let mut updated = true;
        while updated {
            let (r_updated, r_least_paths) = self.least_paths_next(&least_paths, min_l, max_l);
            updated = r_updated;
            least_paths = r_least_paths;
        }
        let mut min_val = usize::MAX;
        for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
            if let Some(val) = least_paths.get(&(0, 0, dir)) {
                min_val = min(min_val, *val);
            }
        }
        min_val
    }

    fn least_paths_next(
        &self,
        prev: &HashMap<(i64, i64, Dir), usize>,
        min_l: i64,
        max_l: i64,
    ) -> (bool, HashMap<(i64, i64, Dir), usize>) {
        let mut next_least_paths: HashMap<(i64, i64, Dir), usize> = HashMap::new();
        let mut updated = false;
        for i in 0..self.height {
            for j in 0..self.width {
                for not_dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
                    let initial_val = *prev.get(&(i, j, not_dir)).unwrap_or(&usize::MAX);
                    let mut min_val = initial_val;
                    for dir in not_dir.turn() {
                        for l in min_l..=max_l {
                            if let Some((n_i, n_j, loss)) = self.go(i, j, dir, l) {
                                if let Some(val) = prev.get(&(n_i, n_j, dir)) {
                                    min_val = min(min_val, *val + loss);
                                }
                            }
                        }
                    }
                    if initial_val != min_val {
                        updated = true;
                    }
                    if min_val != usize::MAX {
                        next_least_paths.insert((i, j, not_dir), min_val);
                    }
                }
            }
        }
        (updated, next_least_paths)
    }

    fn go(&self, i: i64, j: i64, dir: Dir, dist: i64) -> Option<(i64, i64, usize)> {
        let mut n_i = i;
        let mut n_j = j;
        let mut loss = 0;
        for _ in 0..dist {
            (n_i, n_j) = dir.step(n_i, n_j);
            if let Some(block) = self.blocks.get(&(n_i, n_j)) {
                loss += *block as usize;
            } else {
                return None;
            }
        }
        Some((n_i, n_j, loss))
    }
}
