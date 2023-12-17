use std::collections::{HashMap, HashSet};

pub struct Cave {
    rocks: HashSet<(i64, i64)>,
    width: i64,
    height: i64,
}

impl Cave {
    pub fn from_grid(grid: &HashMap<(i64, i64), char>) -> Cave {
        let rocks = grid
            .iter()
            .filter(|(_, ch)| **ch == '#')
            .map(|(id, _)| *id)
            .collect();
        let width = grid.iter().map(|((i, _), _)| i + 1).max().unwrap_or(0);
        let height = grid.iter().map(|((_, j), _)| j + 1).max().unwrap_or(0);
        Cave {
            rocks,
            width,
            height,
        }
    }

    fn has_mirror_col_smudge(&self, num_fail: usize, index: i64) -> bool {
        let count = self
            .rocks
            .iter()
            .filter(|(i, j)| {
                let m_i = 1 + 2 * index - i;
                m_i >= 0 && m_i < self.width && !self.rocks.contains(&(m_i, *j))
            })
            .count();
        count == num_fail
    }

    fn has_mirror_row_smudge(&self, num_fail: usize, index: i64) -> bool {
        let count = self
            .rocks
            .iter()
            .filter(|(i, j)| {
                let m_j = 1 + 2 * index - j;
                m_j >= 0 && m_j < self.height && !self.rocks.contains(&(*i, m_j))
            })
            .count();
        count == num_fail
    }

    fn score(&self, num_fail: usize) -> usize {
        for i in 0..(self.width - 1) {
            if self.has_mirror_col_smudge(num_fail, i) {
                return ((i + 1) * 100) as usize;
            }
        }
        for j in 0..(self.height - 1) {
            if self.has_mirror_row_smudge(num_fail, j) {
                return (j + 1) as usize;
            }
        }
        0
    }
}

pub struct Caves {
    caves: Vec<Cave>,
}

impl Caves {
    pub fn from_grids(grids: &Vec<HashMap<(i64, i64), char>>) -> Caves {
        let caves = grids.iter().map(|grid| Cave::from_grid(grid)).collect();
        Caves { caves }
    }

    pub fn score(&self) -> usize {
        self.caves.iter().map(|cave| cave.score(0)).sum()
    }

    pub fn score_smudge(&self) -> usize {
        self.caves.iter().map(|cave| cave.score(1)).sum()
    }
}
