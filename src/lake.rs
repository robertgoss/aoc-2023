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

fn convert(map: &mut HashMap<(i64, i64), usize>, from: usize, to: usize) {
    for v in map.values_mut() {
        if *v == from {
            *v = to;
        }
    }
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
        let areas = self.connected_areas();
        let max = self.max();
        let inner: usize = areas
            .iter()
            .filter(|area| !area.contains(&max))
            .map(|area| area.len())
            .sum();
        inner + self.points.len()
    }

    fn connected_areas(&self) -> Vec<HashSet<(i64, i64)>> {
        let mut area_nums: HashSet<usize> = HashSet::new();
        let mut area_map: HashMap<(i64, i64), usize> = HashMap::new();
        let max = self.max();
        let min = self.min();
        let mut num = 0_usize;
        for i in min.0 - 1..=max.0 + 1 {
            for j in min.1 - 1..=max.1 + 1 {
                if !self.points.contains(&(i, j)) {
                    area_map.insert((i, j), num);
                    area_nums.insert(num);
                    num += 1;
                }
            }
        }
        // Deduplicate
        for i in min.0 - 1..=max.0 + 1 {
            for j in min.0 - 1..=max.1 + 1 {
                if let Some(&num_base) = area_map.get(&(i, j)) {
                    if let (Some(&num_up), Some(&num_left)) =
                        (area_map.get(&(i - 1, j)), area_map.get(&(i, j - 1)))
                    {
                        // Merge
                        if num_up != num_left {
                            convert(&mut area_map, num_up, num_left);
                            area_nums.remove(&num_up);
                        }
                        area_map.insert((i, j), num_left);
                        area_nums.remove(&num_base);
                    } else {
                        if let Some(&num_up) = area_map.get(&(i - 1, j)) {
                            area_map.insert((i, j), num_up);
                            area_nums.remove(&num_base);
                        } else if let Some(&num_left) = area_map.get(&(i, j - 1)) {
                            area_map.insert((i, j), num_left);
                            area_nums.remove(&num_base);
                        }
                    }
                }
            }
        }
        // Get common
        let mut areas = Vec::new();
        for i in area_nums {
            let mut area: HashSet<(i64, i64)> = HashSet::new();
            for (ind, v) in area_map.iter() {
                if *v == i {
                    area.insert(*ind);
                }
            }
            areas.push(area);
        }
        println!("{} {:?} {:?}", areas.len(), max, min);
        areas
    }

    fn max(&self) -> (i64, i64) {
        let x = self.points.iter().map(|(x, _)| x + 1).max().unwrap_or(1);
        let y = self.points.iter().map(|(_, y)| y + 1).max().unwrap_or(1);
        (x, y)
    }

    fn min(&self) -> (i64, i64) {
        let x = self.points.iter().map(|(x, _)| x - 1).min().unwrap_or(1);
        let y = self.points.iter().map(|(_, y)| y - 1).min().unwrap_or(1);
        (x, y)
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
