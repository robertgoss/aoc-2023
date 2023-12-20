use std::cmp::max;
use std::collections::{HashMap, HashSet};

use super::utils::Dir;

enum Mirror {
    DiagUR,
    DiagUL,
    Horizontal,
    Vertical,
}

pub struct MirrorCave {
    mirrors: HashMap<(i64, i64), Mirror>,
    width: i64,
    height: i64,
}

impl Mirror {
    fn from_char(ch: char) -> Option<Mirror> {
        match ch {
            '/' => Some(Mirror::DiagUR),
            '\\' => Some(Mirror::DiagUL),
            '-' => Some(Mirror::Horizontal),
            '|' => Some(Mirror::Vertical),
            _ => None,
        }
    }
}

impl MirrorCave {
    pub fn from_grid(grid: &HashMap<(i64, i64), char>) -> MirrorCave {
        let mirrors: HashMap<(i64, i64), Mirror> = grid
            .iter()
            .filter_map(|((i, j), ch)| Mirror::from_char(*ch).map(|mir| ((*i, *j), mir)))
            .collect();
        let height = grid.iter().map(|((i, _), _)| i + 1).max().unwrap_or(0);
        let width = grid.iter().map(|((_, j), _)| j + 1).max().unwrap_or(0);
        MirrorCave {
            mirrors,
            width,
            height,
        }
    }

    pub fn simulate_excited(&self) -> usize {
        self.simulate_excited_start(0, 0, Dir::Right)
    }

    fn simulate_excited_start(&self, i: i64, j: i64, dir: Dir) -> usize {
        let mut beams: HashSet<(i64, i64, Dir)> = HashSet::new();
        let mut next: Vec<(i64, i64, Dir)> = vec![(i, j, dir)];
        while !next.is_empty() {
            let (ni, nj, n_dir) = next.pop().unwrap();
            self.simulate_beam(ni, nj, n_dir, &mut beams, &mut next);
        }
        let excited: HashSet<(i64, i64)> = beams.iter().map(|(i, j, _)| (*i, *j)).collect();
        excited.len()
    }

    pub fn max_simulate_excited(&self) -> usize {
        let m_up = self.max_simulate_excited_up();
        let m_down = self.max_simulate_excited_down();
        let m_left = self.max_simulate_excited_left();
        let m_right = self.max_simulate_excited_right();
        max(max(m_up, m_down), max(m_left, m_right))
    }

    fn max_simulate_excited_up(&self) -> usize {
        (0..self.width)
            .map(|j| self.simulate_excited_start(self.height - 1, j, Dir::Up))
            .max()
            .unwrap_or(0)
    }

    fn max_simulate_excited_down(&self) -> usize {
        (0..self.width)
            .map(|j| self.simulate_excited_start(0, j, Dir::Down))
            .max()
            .unwrap_or(0)
    }

    fn max_simulate_excited_left(&self) -> usize {
        (0..self.height)
            .map(|i| self.simulate_excited_start(i, self.width - 1, Dir::Left))
            .max()
            .unwrap_or(0)
    }

    fn max_simulate_excited_right(&self) -> usize {
        (0..self.height)
            .map(|i| self.simulate_excited_start(i, 0, Dir::Right))
            .max()
            .unwrap_or(0)
    }

    fn simulate_beam(
        &self,
        i: i64,
        j: i64,
        dir: Dir,
        beams: &mut HashSet<(i64, i64, Dir)>,
        next: &mut Vec<(i64, i64, Dir)>,
    ) {
        if i < 0 || i >= self.height {
            return;
        }
        if j < 0 || j >= self.width {
            return;
        }
        if beams.contains(&(i, j, dir)) {
            return;
        }
        beams.insert((i, j, dir));
        match self.mirrors.get(&(i, j)) {
            Some(Mirror::DiagUL) => {
                let new_dir = dir.diag_ul();
                let (ni, nj) = new_dir.step(i, j);
                next.push((ni, nj, new_dir));
            }
            Some(Mirror::DiagUR) => {
                let new_dir = dir.diag_ur();
                let (ni, nj) = new_dir.step(i, j);
                next.push((ni, nj, new_dir));
            }
            Some(Mirror::Horizontal) => {
                if dir == Dir::Up || dir == Dir::Down {
                    next.push((i, j - 1, Dir::Left));
                    next.push((i, j + 1, Dir::Right));
                } else {
                    let (ni, nj) = dir.step(i, j);
                    next.push((ni, nj, dir));
                }
            }
            Some(Mirror::Vertical) => {
                if dir == Dir::Left || dir == Dir::Right {
                    next.push((i - 1, j, Dir::Up));
                    next.push((i + 1, j, Dir::Down));
                } else {
                    let (ni, nj) = dir.step(i, j);
                    next.push((ni, nj, dir));
                }
            }
            None => {
                let (ni, nj) = dir.step(i, j);
                next.push((ni, nj, dir));
            }
        }
    }
}
