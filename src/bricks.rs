use std::collections::{BTreeSet, HashSet};

struct Brick {
    start: (i64, i64, i64),
    end: (i64, i64, i64),
}

pub struct Bricks {
    bricks: Vec<Brick>,
}

fn point(string: &str) -> Option<(i64, i64, i64)> {
    let mut parts = string.split(",");
    let x = parts.next().and_then(|s| s.parse::<i64>().ok())?;
    let y = parts.next().and_then(|s| s.parse::<i64>().ok())?;
    let z = parts.next().and_then(|s| s.parse::<i64>().ok())?;
    Some((x, y, z))
}

fn chain_count(
    dependencies: &Vec<Vec<usize>>,
    reverse_dependencies: &Vec<Vec<usize>>,
    start: usize,
) -> usize {
    let mut count = 0;
    let mut to_remove = BTreeSet::new();
    to_remove.insert(start);
    let mut removed = HashSet::new();
    while !to_remove.is_empty() {
        let remove = to_remove.pop_first().unwrap();
        removed.insert(remove);
        count += 1;
        for above in &reverse_dependencies[remove] {
            if !removed.contains(above) {
                let unsupported = dependencies[*above].iter().all(|dep| removed.contains(dep));
                if unsupported {
                    to_remove.insert(*above);
                }
            }
        }
    }
    count - 1
}

impl Bricks {
    pub fn from_lines(lines: &Vec<String>) -> Option<Bricks> {
        let bricks = lines
            .iter()
            .map(|line| Brick::from_line(line))
            .collect::<Option<Vec<Brick>>>()?;
        Some(Bricks { bricks })
    }

    pub fn disintergrate_count(&mut self) -> usize {
        self.drop_down();
        let (dependencies, reverse_dependencies) = self.depended_maps();
        let mut count = 0_usize;
        for rev_d in reverse_dependencies.iter() {
            if rev_d.iter().all(|upper| dependencies[*upper].len() > 1) {
                count += 1;
            }
        }
        count
    }

    pub fn maximum_chain(&mut self) -> usize {
        self.drop_down();
        let (dependencies, reverse_dependencies) = self.depended_maps();
        (0..self.bricks.len())
            .map(|i| chain_count(&dependencies, &reverse_dependencies, i))
            .sum()
    }

    fn depended_maps(&self) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
        let dependencies: Vec<Vec<usize>> = self
            .bricks
            .iter()
            .map(|brick| self.dependencies(brick))
            .collect();
        let mut reverse_dependencies: Vec<Vec<usize>> = vec![Vec::new(); self.bricks.len()];
        for (i, dep) in dependencies.iter().enumerate() {
            for j in dep {
                reverse_dependencies[*j].push(i);
            }
        }
        (dependencies, reverse_dependencies)
    }

    fn drop_down(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;
            let mut to_move = Vec::new();
            for (i, brick) in self.bricks.iter().enumerate() {
                if self.holding_count(brick) == 0 && brick.start.2 > 0 {
                    to_move.push(i);
                    moved = true;
                }
            }
            for i in to_move {
                self.bricks[i].move_down();
            }
        }
    }

    fn holding_count(&self, brick: &Brick) -> usize {
        self.dependencies(brick).len()
    }

    fn dependencies(&self, brick: &Brick) -> Vec<usize> {
        let below = brick.below();
        self.bricks
            .iter()
            .enumerate()
            .filter_map(|(i, other)| other.overlap(&below).then_some(i))
            .collect()
    }
}

impl Brick {
    pub fn from_line(line: &str) -> Option<Brick> {
        let (start_str, end_str) = line.split_once("~")?;
        let start = point(start_str)?;
        let end = point(end_str)?;
        Some(Brick { start, end })
    }

    fn move_down(&mut self) {
        self.start.2 -= 1;
        self.end.2 -= 1;
    }

    fn below(&self) -> Brick {
        Brick {
            start: (self.start.0, self.start.1, self.start.2 - 1),
            end: (self.end.0, self.end.1, self.start.2 - 1),
        }
    }

    fn overlap(&self, other: &Brick) -> bool {
        self.overlap_x(other) && self.overlap_y(other) && self.overlap_z(other)
    }

    fn overlap_x(&self, other: &Brick) -> bool {
        other.start.0 <= self.end.0 && other.end.0 >= self.start.0
    }

    fn overlap_y(&self, other: &Brick) -> bool {
        other.start.1 <= self.end.1 && other.end.1 >= self.start.1
    }

    fn overlap_z(&self, other: &Brick) -> bool {
        other.start.2 <= self.end.2 && other.end.2 >= self.start.2
    }
}
