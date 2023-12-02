use std::cmp::max;
use std::collections::HashMap;

struct Round {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

struct Game {
    rounds: Vec<Round>,
}

pub struct Games {
    games: HashMap<u32, Game>,
}

fn max_merge_val(o1: &Option<u32>, o2: &Option<u32>) -> Option<u32> {
    if let Some(v1) = o1 {
        if let Some(v2) = o2 {
            Some(max(*v1, *v2))
        } else {
            Some(*v1)
        }
    } else {
        if let Some(v2) = o2 {
            Some(*v2)
        } else {
            None
        }
    }
}

impl Round {
    fn empty() -> Round {
        Round {
            red: None,
            green: None,
            blue: None,
        }
    }
    fn from_string(string: &str) -> Option<Round> {
        let mut round = Round::empty();
        for part in string.split(", ") {
            let (value_str, name_str) = part.split_once(' ')?;
            let value = value_str.parse::<u32>().ok()?;
            match name_str {
                "red" => round.add_red(value),
                "blue" => round.add_blue(value),
                "green" => round.add_green(value),
                _ => return None,
            }
        }
        Some(round)
    }

    fn add_red(&mut self, val: u32) {
        *self.red.get_or_insert(0) += val;
    }

    fn add_blue(&mut self, val: u32) {
        *self.blue.get_or_insert(0) += val;
    }

    fn add_green(&mut self, val: u32) {
        *self.green.get_or_insert(0) += val;
    }

    fn possible(&self) -> bool {
        if let Some(r) = self.red {
            if r > 12 {
                return false;
            }
        }
        if let Some(g) = self.green {
            if g > 13 {
                return false;
            }
        }
        if let Some(b) = self.blue {
            if b > 14 {
                return false;
            }
        }
        true
    }

    fn max_merge(&self, other: &Round) -> Round {
        Round {
            red: max_merge_val(&self.red, &other.red),
            green: max_merge_val(&self.green, &other.green),
            blue: max_merge_val(&self.blue, &other.blue),
        }
    }

    fn power_set(&self) -> u64 {
        self.red.unwrap_or(0) as u64
            * self.green.unwrap_or(0) as u64
            * self.blue.unwrap_or(0) as u64
    }
}

impl Game {
    fn from_string(string: &str) -> Option<Game> {
        let rounds = string
            .split("; ")
            .map(|part| Round::from_string(part))
            .collect::<Option<Vec<Round>>>()?;
        Some(Game { rounds })
    }

    fn possible(&self) -> bool {
        self.rounds.iter().all(|round| round.possible())
    }

    fn max_merge_round(&self) -> Round {
        self.rounds
            .iter()
            .fold(Round::empty(), |a, b| a.max_merge(b))
    }
}

impl Games {
    pub fn from_lines(lines: &Vec<String>) -> Option<Games> {
        let mut games = HashMap::new();
        for line in lines {
            let (pref_index_str, game_str) = line.split_once(": ")?;
            let index_str = pref_index_str.strip_prefix("Game ")?;
            let index = index_str.parse::<u32>().ok()?;
            let game = Game::from_string(game_str)?;
            games.insert(index, game);
        }
        Some(Games { games })
    }

    pub fn sum_power_set(&self) -> u64 {
        self.games
            .values()
            .map(|game| game.max_merge_round().power_set())
            .sum()
    }

    pub fn sum_possible(&self) -> u32 {
        self.games
            .iter()
            .filter(|(_, b)| b.possible())
            .map(|(a, _)| *a)
            .sum()
    }
}
