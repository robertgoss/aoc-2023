use num::Integer;
use std::collections::HashMap;

enum Dir {
    R,
    L,
}

impl Dir {
    fn from_char(ch: char) -> Option<Dir> {
        match ch {
            'L' => Some(Dir::L),
            'R' => Some(Dir::R),
            _ => None,
        }
    }
}

pub struct Map {
    directions: Vec<Dir>,
    map: HashMap<String, (String, String)>,
}

fn split_line(line: &str) -> Option<(String, String, String)> {
    let (source, target) = line.split_once(" = (")?;
    let (left, right_b) = target.split_once(", ")?;
    let right = right_b.strip_suffix(")")?;
    Some((source.to_string(), left.to_string(), right.to_string()))
}

impl Map {
    pub fn from_string(string: &String) -> Option<Map> {
        let (dir_str, map_str) = string.split_once("\n\n")?;
        let directions = dir_str
            .chars()
            .map(|ch| Dir::from_char(ch))
            .collect::<Option<Vec<Dir>>>()?;
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        for line in map_str.lines() {
            let (source, left, right) = split_line(line)?;
            map.insert(source, (left, right));
        }
        Some(Map { directions, map })
    }

    pub fn num_steps(&self) -> usize {
        let mut state = "AAA".to_string();
        for (i, dir) in self.directions.iter().cycle().enumerate() {
            if state == "ZZZ" {
                return i;
            }
            state = self.next_state(&state, dir);
        }
        0
    }

    pub fn num_steps_ghost(&self) -> usize {
        let counts: Vec<usize> = self
            .ghost_start()
            .into_iter()
            .map(|state| self.num_steps_ghost_single(&state))
            .collect();
        counts
            .iter()
            .fold(*counts.first().unwrap_or(&0), |a, b| a.lcm(b))
    }

    fn num_steps_ghost_single(&self, init: &String) -> usize {
        let mut state: String = init.clone();
        for (i, dir) in self.directions.iter().cycle().enumerate() {
            if self.is_end(&state) {
                return i;
            }
            state = self.next_state(&state, dir);
        }
        0
    }

    fn ghost_start(&self) -> Vec<String> {
        self.map
            .keys()
            .filter(|state| state.chars().nth(2) == Some('A'))
            .cloned()
            .collect()
    }

    fn is_end(&self, state: &String) -> bool {
        state.chars().nth(2) == Some('Z')
    }

    fn next_state(&self, state: &String, dir: &Dir) -> String {
        let (left, right) = self.map.get(state).unwrap();
        match dir {
            Dir::L => left.clone(),
            Dir::R => right.clone(),
        }
    }
}
