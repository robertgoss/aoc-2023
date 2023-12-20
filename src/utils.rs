#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    pub fn step(&self, i: i64, j: i64) -> (i64, i64) {
        self.go(i, j, 1)
    }

    pub fn go(&self, i: i64, j: i64, d: i64) -> (i64, i64) {
        match self {
            Dir::Up => (i - d, j),
            Dir::Down => (i + d, j),
            Dir::Left => (i, j - d),
            Dir::Right => (i, j + d),
        }
    }

    pub fn diag_ul(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    pub fn diag_ur(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    pub fn turn(&self) -> [Dir; 2] {
        match self {
            Dir::Up => [Dir::Left, Dir::Right],
            Dir::Down => [Dir::Left, Dir::Right],
            Dir::Left => [Dir::Up, Dir::Down],
            Dir::Right => [Dir::Up, Dir::Down],
        }
    }
}
