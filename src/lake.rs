use crate::utils::Dir;
use itertools::Itertools;
use num::traits::FloatConst;
use std::collections::{HashMap, HashSet};

struct Move {
    dir: Dir,
    dist: i64,
}

pub struct Path {
    points: HashSet<(i64, i64)>,
    vertices: Vec<(i64, i64)>,
}

impl Path {
    pub fn from_lines(lines: &Vec<String>) -> Option<Path> {
        let moves = lines
            .iter()
            .map(|line| Move::from_line(line))
            .collect::<Option<Vec<Move>>>()?;
        let mut points = HashSet::new();
        let mut vertices = Vec::new();
        let mut start: (i64, i64) = (0, 0);
        points.insert(start);
        vertices.push(start);
        for movement in &moves {
            for l in 1..movement.dist {
                points.insert(movement.dir.go(start.0, start.1, l));
            }
            let end = movement.dir.go(start.0, start.1, movement.dist);
            points.insert(end);
            vertices.push(end);
            start = end;
        }
        Some(Path { points, vertices })
    }

    pub fn area(&self) -> usize {
        let (width, height) = self.max();
        (0..=width)
            .cartesian_product(0..=height)
            .filter(|(i, j)| self.contains(*i, *j))
            .count()
    }

    fn max(&self) -> (i64, i64) {
        let x = self.points.iter().map(|(x, _)| *x).max().unwrap_or(1);
        let y = self.points.iter().map(|(_, y)| *y).max().unwrap_or(1);
        (x, y)
    }

    fn contains(&self, i: i64, j: i64) -> bool {
        if self.points.contains(&(i, j)) {
            return true;
        }
        let mut winding: f64 = 0.0;
        let mut prev_angle: Option<f64> = None;
        for point in &self.vertices {
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
}

fn dir_from_string(string: &str) -> Option<Dir> {
    match string {
        "R" => Some(Dir::Right),
        "L" => Some(Dir::Left),
        "U" => Some(Dir::Up),
        "D" => Some(Dir::Down),
        _ => None,
    }
}

impl Move {
    fn from_line(line: &str) -> Option<Move> {
        let mut parts = line.split(" ");
        let dir_s = parts.next()?;
        let dir = dir_from_string(dir_s)?;
        let dist_s = parts.next()?;
        let dist = dist_s.parse::<i64>().ok()?;
        Some(Move { dir, dist })
    }
}
