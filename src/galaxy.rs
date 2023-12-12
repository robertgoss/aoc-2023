use std::collections::{BTreeSet, HashMap, HashSet};

pub struct Galaxy {
    stars: HashSet<(i64, i64)>,
}

fn expand(expansion: usize, vals: &BTreeSet<i64>) -> HashMap<i64, i64> {
    let mut prev = vals.first().unwrap() - 1;
    let multiplier = (expansion - 1) as i64;
    let mut offset = 0;
    let mut map: HashMap<i64, i64> = HashMap::new();
    for val in vals {
        if *val != prev + 1 {
            offset += (val - prev) - 1;
        }
        map.insert(*val, val + offset * multiplier);
        prev = *val;
    }
    map
}

impl Galaxy {
    pub fn from_grid(expansion: usize, grid: &HashMap<(i64, i64), char>) -> Galaxy {
        let stars = grid
            .iter()
            .filter(|(_, ch)| **ch == '#')
            .map(|(id, _)| *id)
            .collect();
        (Galaxy { stars }).expand_x(expansion).expand_y(expansion)
    }

    pub fn sum_dist(&self) -> usize {
        let mut sum: usize = 0;
        for star1 in &self.stars {
            for star2 in &self.stars {
                if *star1 < *star2 {
                    sum += self.dist(star1, star2);
                }
            }
        }
        sum
    }

    fn dist(&self, star1: &(i64, i64), star2: &(i64, i64)) -> usize {
        (star2.0 - star1.0).abs() as usize + (star2.1 - star1.1).abs() as usize
    }

    fn expand_x(&self, expansion: usize) -> Galaxy {
        let x_vals: BTreeSet<i64> = self.stars.iter().map(|(x, _)| *x).collect();
        let map = expand(expansion, &x_vals);
        let new_stars = self
            .stars
            .iter()
            .map(|(x, y)| (*map.get(x).unwrap(), *y))
            .collect();
        Galaxy { stars: new_stars }
    }

    fn expand_y(&self, expansion: usize) -> Galaxy {
        let y_vals: BTreeSet<i64> = self.stars.iter().map(|(_, y)| *y).collect();
        let map = expand(expansion, &y_vals);
        let new_stars = self
            .stars
            .iter()
            .map(|(x, y)| (*x, *map.get(y).unwrap()))
            .collect();
        Galaxy { stars: new_stars }
    }
}
