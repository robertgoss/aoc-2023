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
}

impl Pipes {
    pub fn from_grid(grid: &HashMap<(i64, i64), char>) -> Pipes {
        let pipes: HashMap<(i64, i64), Pipe> = grid
            .iter()
            .filter_map(|((i, j), ch)| Pipe::from_char(*ch).map(|pipe| ((*i, *j), pipe)))
            .collect();
        let max_i = pipes.keys().map(|(i, _)| *i).max().unwrap_or(0);
        Pipes { pipes, max_i }
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
        let (_, pipe_loop) = self.pipe_loop();
        self.pipes
            .keys()
            .filter(|(i, j)| self.inside_loop(*i, *j, &pipe_loop))
            .count()
    }

    fn inside_loop(&self, i: i64, j: i64, pipe_loop: &HashSet<(i64, i64)>) -> bool {
        if pipe_loop.contains(&(i, j)) {
            return false;
        }
        let mut sign = false;
        for k in (i + 1)..=self.max_i {
            // Need more complex state machine to deal with vertical stacks!
            if pipe_loop.contains(&(k, j)) {
                sign = !sign;
            }
        }
        sign
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
