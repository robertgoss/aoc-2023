use num::rational::Ratio;
use num::Zero;

struct Hail {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

pub struct Storm {
    hail: Vec<Hail>,
}

fn vec3(string: &str) -> Option<(i64, i64, i64)> {
    let mut parts = string.split(", ");
    let x = parts
        .next()
        .and_then(|s| s.trim_start().parse::<i64>().ok())?;
    let y = parts
        .next()
        .and_then(|s| s.trim_start().parse::<i64>().ok())?;
    let z = parts
        .next()
        .and_then(|s| s.trim_start().parse::<i64>().ok())?;
    Some((x, y, z))
}

impl Storm {
    pub fn from_lines(lines: &Vec<String>) -> Option<Storm> {
        let hail = lines
            .iter()
            .map(|line| Hail::from_line(line))
            .collect::<Option<Vec<Hail>>>()?;
        Some(Storm { hail })
    }

    pub fn intersect_in_area(&self, min_v: i64, max_v: i64) -> usize {
        let area_min = (min_v, min_v);
        let area_max = (max_v, max_v);
        let rat_zero: Ratio<i128> = Ratio::zero();
        let mut count = 0;
        for (i, hail1) in self.hail.iter().enumerate() {
            for hail2 in self.hail.iter().skip(i) {
                if let Some((t, s)) = hail1.intersect(hail2) {
                    if t >= rat_zero && s >= rat_zero {
                        if hail1.in_area(t, area_min, area_max)
                            && hail2.in_area(t, area_min, area_max)
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

impl Hail {
    fn from_line(line: &str) -> Option<Hail> {
        let (pos_s, vec_s) = line.split_once(" @ ")?;
        let pos = vec3(pos_s)?;
        let vel = vec3(vec_s)?;
        Some(Hail { pos, vel })
    }

    fn intersect(&self, other: &Hail) -> Option<(Ratio<i128>, Ratio<i128>)> {
        let v_a = (self.vel.0 as i128, self.vel.1 as i128);
        let v_b_in = (other.vel.1 as i128, -other.vel.0 as i128);
        let denom = (v_a.0 * v_b_in.0) + (v_a.1 * v_b_in.1);
        if denom == 0 {
            return None;
        }
        let diff = (
            (other.pos.0 - self.pos.0) as i128,
            (other.pos.1 - self.pos.1) as i128,
        );
        let num_t = (diff.0 * v_b_in.0) + (diff.1 * v_b_in.1);
        let t = Ratio::new(num_t, denom);
        let v_a_in = (self.vel.1 as i128, -self.vel.0 as i128);
        let num_s = -(diff.0 * v_a_in.0) - (diff.1 * v_a_in.1);
        let s = Ratio::new(-num_s, denom);
        assert_eq!(self.eval(t).0, other.eval(s).0);
        assert_eq!(self.eval(t).1, other.eval(s).1);
        Some((t, s))
    }

    fn in_area(&self, t: Ratio<i128>, min: (i64, i64), max: (i64, i64)) -> bool {
        let (x, y, _) = self.eval(t);
        let min_x = Ratio::from_integer(min.0 as i128);
        let min_y = Ratio::from_integer(min.1 as i128);
        let max_x = Ratio::from_integer(max.0 as i128);
        let max_y = Ratio::from_integer(max.1 as i128);
        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }

    fn eval(&self, t: Ratio<i128>) -> (Ratio<i128>, Ratio<i128>, Ratio<i128>) {
        let x =
            Ratio::from_integer(self.pos.0 as i128) + (t * Ratio::from_integer(self.vel.0 as i128));
        let y =
            Ratio::from_integer(self.pos.1 as i128) + (t * Ratio::from_integer(self.vel.1 as i128));
        let z =
            Ratio::from_integer(self.pos.2 as i128) + (t * Ratio::from_integer(self.vel.2 as i128));
        (x, y, z)
    }
}
