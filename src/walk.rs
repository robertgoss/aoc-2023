use petgraph::algo::all_simple_paths;
use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;

enum Tile {
    Empty,
    N,
    S,
    E,
    W,
}

pub struct Woods {
    tiles: HashMap<(i64, i64), Tile>,
}

impl Tile {
    fn from_char(ch: &char) -> Option<Tile> {
        match ch {
            '>' => Some(Tile::E),
            '^' => Some(Tile::N),
            '<' => Some(Tile::W),
            'v' => Some(Tile::S),
            '.' => Some(Tile::Empty),
            _ => None,
        }
    }

    fn next(&self, i: i64, j: i64, uphill: bool) -> Vec<(i64, i64)> {
        if uphill {
            return vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
        }
        match self {
            Tile::N => vec![(i - 1, j)],
            Tile::S => vec![(i + 1, j)],
            Tile::W => vec![(i, j - 1)],
            Tile::E => vec![(i, j + 1)],
            Tile::Empty => vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)],
        }
    }
}

impl Woods {
    pub fn from_grid(grid: &HashMap<(i64, i64), char>) -> Woods {
        let tiles = grid
            .iter()
            .filter_map(|(ind, ch)| Tile::from_char(ch).map(|t| (*ind, t)))
            .collect();
        Woods { tiles }
    }

    pub fn maximum_path(&self, uphill: bool) -> usize {
        let graph = self.graph(uphill);
        let start = self.start();
        let end = self.end();
        all_simple_paths::<Vec<_>, _>(&graph, start, end, 0, None)
            .map(|i| i.len())
            .max()
            .unwrap()
            - 1
    }

    fn start(&self) -> (i64, i64) {
        *self.tiles.keys().min_by_key(|(i, _)| i).unwrap()
    }

    fn end(&self) -> (i64, i64) {
        *self.tiles.keys().max_by_key(|(i, _)| i).unwrap()
    }

    fn graph(&self, uphill: bool) -> DiGraphMap<(i64, i64), ()> {
        let mut graph = DiGraphMap::new();
        for ((i, j), t) in &self.tiles {
            for (ni, nj) in t.next(*i, *j, uphill) {
                if self.tiles.contains_key(&(ni, nj)) {
                    graph.add_edge((*i, *j), (ni, nj), ());
                }
            }
        }
        graph
    }
}
