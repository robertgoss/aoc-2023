use itertools::Itertools;
use num::traits::FloatConst;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq)]
enum Pipe {
    Start,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
}

impl Pipe {
    fn from_char(ch: char) -> Option<Pipe> {
        match ch {
            'S' => Some(Pipe::Start),
            '|' => Some(Pipe::NS),
            '-' => Some(Pipe::EW),
            'L' => Some(Pipe::NE),
            'J' => Some(Pipe::NW),
            '7' => Some(Pipe::SW),
            'F' => Some(Pipe::SE),
            _ => None,
        }
    }
}

impl Pipe {
    fn connect_south(&self) -> bool {
        match self {
            Pipe::Start => true,
            Pipe::NS => true,
            Pipe::SE => true,
            Pipe::SW => true,
            _ => false,
        }
    }

    fn connect_north(&self) -> bool {
        match self {
            Pipe::Start => true,
            Pipe::NS => true,
            Pipe::NE => true,
            Pipe::NW => true,
            _ => false,
        }
    }

    fn connect_east(&self) -> bool {
        match self {
            Pipe::Start => true,
            Pipe::EW => true,
            Pipe::SE => true,
            Pipe::NE => true,
            _ => false,
        }
    }

    fn connect_west(&self) -> bool {
        match self {
            Pipe::Start => true,
            Pipe::EW => true,
            Pipe::SW => true,
            Pipe::NW => true,
            _ => false,
        }
    }
}

pub struct Pipes {
    pipes: HashMap<(i64, i64), Pipe>,
    max_i: i64,
    max_j: i64,
}

impl Pipes {
    pub fn from_grid(grid: &HashMap<(i64, i64), char>) -> Pipes {
        let pipes: HashMap<(i64, i64), Pipe> = grid
            .iter()
            .filter_map(|((i, j), ch)| Pipe::from_char(*ch).map(|pipe| ((*i, *j), pipe)))
            .collect();
        let max_i = pipes.keys().map(|(i, _)| *i).max().unwrap_or(0);
        let max_j = pipes.keys().map(|(_, j)| *j).max().unwrap_or(0);
        Pipes {
            pipes,
            max_i,
            max_j,
        }
    }

    fn start(&self) -> (i64, i64) {
        let ((i, j), _) = self
            .pipes
            .iter()
            .filter(|(_, pipe)| **pipe == Pipe::Start)
            .next()
            .unwrap();
        (*i, *j)
    }

    pub fn max_distance(&self) -> usize {
        let (index, _) = self.pipe_loop();
        index - 1
    }

    pub fn pipe_loop(&self) -> (usize, HashSet<(i64, i64)>) {
        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let mut current: HashSet<(i64, i64)> = HashSet::new();
        current.insert(self.start());
        let mut index: usize = 0;
        while !current.is_empty() {
            let connected = self.connected_union(&current);
            for x in current {
                seen.insert(x);
            }
            current = connected.difference(&seen).cloned().collect();
            index += 1;
        }
        (index, seen)
    }

    pub fn enclosed(&self) -> usize {
        let (_, pipe_set) = self.pipe_loop();
        let pipe_loop = self.pipe_loop_ordered();
        //assert_eq!(pipe_set.len(), pipe_loop.len() - 1);
        (0..=self.max_i)
            .cartesian_product(0..=self.max_j)
            .filter(|(i, j)| !pipe_set.contains(&(*i, *j)) && self.inside_loop(*i, *j, &pipe_loop))
            .count()
    }

    fn pipe_loop_ordered(&self) -> Vec<(i64, i64)> {
        let mut ordered = Vec::new();
        let start = self.start();
        let mut prev = start;
        let mut curr = self.next(prev, None);
        ordered.push(start);
        while curr != start {
            ordered.push(curr);
            let next = self.next(curr, Some(prev));
            prev = curr;
            curr = next;
        }
        ordered.push(curr);
        ordered
    }

    fn next(&self, (i, j): (i64, i64), prev: Option<(i64, i64)>) -> (i64, i64) {
        let mut vec: Vec<(i64, i64)> = Vec::new();
        match self.pipes.get(&(i, j)).unwrap() {
            Pipe::Start => {
                vec = self.connected(i, j);
            }
            Pipe::NS => {
                vec.push((i + 1, j));
                vec.push((i - 1, j));
            }
            Pipe::EW => {
                vec.push((i, j + 1));
                vec.push((i, j - 1));
            }
            Pipe::NE => {
                vec.push((i - 1, j));
                vec.push((i, j + 1));
            }
            Pipe::NW => {
                vec.push((i - 1, j));
                vec.push((i, j - 1));
            }
            Pipe::SE => {
                vec.push((i + 1, j));
                vec.push((i, j + 1));
            }
            Pipe::SW => {
                vec.push((i + 1, j));
                vec.push((i, j - 1));
            }
        }
        assert_eq!(vec.len(), 2);
        if let Some(prev_i) = prev {
            if vec[0] == prev_i {
                vec[1]
            } else {
                vec[0]
            }
        } else {
            vec[0]
        }
    }

    fn inside_loop(&self, i: i64, j: i64, pipe_loop: &Vec<(i64, i64)>) -> bool {
        let mut winding: f64 = 0.0;
        let mut prev_angle: Option<f64> = None;
        for point in pipe_loop.iter() {
            let vec = ((point.0 - i) as f64, (point.1 - j) as f64);
            let angle = f64::atan2(vec.1, vec.0);
            if let Some(prev_val) = prev_angle {
                let mut diff = angle - prev_val;
                if diff > f64::PI() {
                    diff -= 2.0 * f64::PI();
                } else if diff < -f64::PI() {
                    diff += 2.0 * f64::PI();
                }
                winding += diff;
            }
            prev_angle = Some(angle);
        }
        return winding.abs() > 1e-6;
    }

    fn connected_union(&self, elems: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
        let mut connected: HashSet<(i64, i64)> = HashSet::new();
        for elem in elems.iter() {
            for next in self.connected(elem.0, elem.1) {
                connected.insert(next);
            }
        }
        connected
    }

    fn connected(&self, i: i64, j: i64) -> Vec<(i64, i64)> {
        let mut found: Vec<(i64, i64)> = Vec::new();
        if let Some(north) = self.pipes.get(&(i - 1, j)) {
            if north.connect_south() {
                found.push((i - 1, j))
            }
        }
        if let Some(south) = self.pipes.get(&(i + 1, j)) {
            if south.connect_north() {
                found.push((i + 1, j))
            }
        }
        if let Some(west) = self.pipes.get(&(i, j - 1)) {
            if west.connect_east() {
                found.push((i, j - 1))
            }
        }
        if let Some(east) = self.pipes.get(&(i, j + 1)) {
            if east.connect_west() {
                found.push((i, j + 1))
            }
        }

        found
    }
}
