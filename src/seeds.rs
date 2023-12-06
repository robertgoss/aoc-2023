struct Ranges {
    vals: Vec<(usize, usize)>,
}
struct RangeMap {
    map: Vec<(usize, usize, usize)>,
}

pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<RangeMap>,
}

impl Ranges {
    fn empty() -> Ranges {
        Ranges { vals: Vec::new() }
    }

    fn add(&mut self, base: usize, len: usize) {
        self.vals.push((base, base + len))
    }

    fn merge(&mut self, other: &Ranges) {
        for (min, max) in &other.vals {
            self.vals.push((*min, *max));
        }
    }

    fn min(&self) -> usize {
        self.vals.iter().map(|(x, _)| *x).min().unwrap()
    }

    fn max(&self) -> usize {
        self.vals.iter().map(|(_, x)| *x).max().unwrap()
    }
}

impl RangeMap {
    fn from_string(string: &str) -> Option<RangeMap> {
        let lines = string.lines().skip(1);
        let mut map: Vec<(usize, usize, usize)> = Vec::new();
        for line in lines {
            let mut parts = line.split(" ");
            let dest_str = parts.next()?;
            let dest = dest_str.parse::<usize>().ok()?;
            let source_str = parts.next()?;
            let source = source_str.parse::<usize>().ok()?;
            let len_str = parts.next()?;
            let len = len_str.parse::<usize>().ok()?;
            map.push((source, dest, len));
        }
        map.sort();
        Some(RangeMap { map })
    }

    fn map(&self, val: usize) -> usize {
        for (source, dest, len) in &self.map {
            if *source <= val && val < source + len {
                let offset = val - source;
                return dest + offset;
            }
        }
        val
    }

    fn map_range(&self, base: usize, len: usize) -> Ranges {
        let max = base + len;
        let mut mapped = Ranges::empty();
        let mut last_max: Option<usize> = None;
        for (source, dest, map_len) in &self.map {
            if source + map_len <= base {
                continue;
            }
            if *source >= max {
                continue;
            }
            if let Some(last_max_val) = last_max {
                if last_max_val < *source {
                    // Add direct map between
                    mapped.add(last_max_val, source - last_max_val);
                }
                let local_max = std::cmp::max(source + map_len, max);
                // Add the mapped range
                mapped.add(*dest, local_max - source);
                last_max = Some(local_max);
            } else {
                if *source > base {
                    mapped.add(base, source - base);
                    let local_max = std::cmp::max(source + map_len, max);
                    // Add the mapped range
                    mapped.add(*dest, local_max - source);
                    last_max = Some(local_max);
                } else {
                    let offset = base - source;
                    // Add the mapped range
                    let local_max = std::cmp::max(source + map_len, max);
                    mapped.add(*dest + offset, local_max - base);
                    last_max = Some(local_max);
                }
            }
        }
        if let Some(last_max_val) = last_max {
            if last_max_val < max {
                mapped.add(last_max_val, max - last_max_val);
            }
        } else {
            mapped.add(base, len);
        }
        mapped
    }

    fn map_ranges(&self, ranges: &Ranges) -> Ranges {
        let mut mapped = Ranges::empty();
        for (base, len) in &ranges.vals {
            mapped.merge(&self.map_range(*base, *len))
        }
        mapped
    }
}

impl Almanac {
    pub fn from_string(string: &str) -> Option<Almanac> {
        let mut chunks = string.split("\n\n");
        let seeds_line = chunks.next()?;
        let seeds_str = seeds_line.strip_prefix("seeds: ")?;
        let seeds = seeds_str
            .split(" ")
            .map(|seed_str| seed_str.parse::<usize>().ok())
            .collect::<Option<Vec<usize>>>()?;
        let maps = chunks
            .map(|chunk| RangeMap::from_string(chunk))
            .collect::<Option<Vec<RangeMap>>>()?;
        Some(Almanac { seeds, maps })
    }

    fn seed_to_site(&self, seed: usize) -> usize {
        let mut val = seed;
        for map in &self.maps {
            val = map.map(val);
        }
        val
    }

    fn seed_range_to_site(&self, base: usize, len: usize) -> usize {
        let mut val = Ranges::empty();
        val.add(base, len);
        for map in &self.maps {
            val = map.map_ranges(&val);
        }
        val.min()
    }

    pub fn least_site(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.seed_to_site(*seed))
            .min()
            .unwrap()
    }

    pub fn least_site_ranges(&self) -> usize {
        self.seeds
            .chunks(2)
            .map(|seed_chunk| self.seed_range_to_site(seed_chunk[0], seed_chunk[1]))
            .min()
            .unwrap()
    }
}
