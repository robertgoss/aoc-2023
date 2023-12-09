use std::cmp::max_by;

pub struct Race {
    time: usize,
    distance: usize,
}

fn merge_line(line: &str) -> Option<usize> {
    let num_str = line
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect::<String>();
    num_str.parse::<usize>().ok()
}
impl Race {
    pub fn num_success(&self) -> usize {
        if let Some((min, max)) = self.range() {
            let v = 1 + max - min;
            v
        } else {
            0
        }
    }

    fn range(&self) -> Option<(usize, usize)> {
        let t = self.time as f64;
        let d = self.distance as f64;
        let discriminant: f64 = (t * t) - (4.0 * d);
        if discriminant < 0.0 {
            return None;
        }
        let root = discriminant.sqrt();
        let min = (t - root) * 0.5;
        let max = (t + root) * 0.5;
        let min_int = min.floor() as usize + 1;
        let max_int = max.ceil() as usize - 1;
        if min_int > max_int {
            return None;
        }
        Some((min_int, max_int))
    }

    pub fn from_lines(lines: &Vec<String>) -> Option<Race> {
        let time_line = lines.get(0)?;
        let time_str = time_line.strip_prefix("Time:")?;
        let time = merge_line(time_str)?;
        let dist_line = lines.get(1)?;
        let dist_str = dist_line.strip_prefix("Distance:")?;
        let distance = merge_line(dist_str)?;
        Some(Race { time, distance })
    }
}

pub struct Races {
    races: Vec<Race>,
}

fn split_line(line: &str) -> Option<Vec<usize>> {
    line.split(" ")
        .filter(|s| !s.is_empty())
        .map(|time| time.parse::<usize>().ok())
        .collect::<Option<Vec<usize>>>()
}

impl Races {
    pub fn from_lines(lines: &Vec<String>) -> Option<Races> {
        let time_line = lines.get(0)?;
        let time_str = time_line.strip_prefix("Time:")?;
        let times = split_line(time_str)?;
        let dist_line = lines.get(1)?;
        let dist_str = dist_line.strip_prefix("Distance:")?;
        let dists = split_line(dist_str)?;
        if times.len() != dists.len() {
            return None;
        }
        let races = times
            .iter()
            .zip(dists.iter())
            .map(|(&time, &distance)| Race { time, distance })
            .collect();
        Some(Races { races })
    }

    pub fn score(&self) -> usize {
        self.races.iter().map(|race| race.num_success()).product()
    }
}
