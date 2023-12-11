use itertools::Itertools;

struct SequenceGen {
    initial_state: Vec<i64>,
    length: usize,
}

fn delta(seq: &Vec<i64>) -> Vec<i64> {
    seq.iter().tuple_windows().map(|(i, j)| j - i).collect()
}

impl SequenceGen {
    pub fn from_line(line: &str) -> Option<SequenceGen> {
        let seq = line
            .split(" ")
            .map(|part| part.parse::<i64>().ok())
            .collect::<Option<Vec<i64>>>()?;
        Some(SequenceGen::from_sequence(&seq))
    }

    fn from_sequence(sequence: &Vec<i64>) -> SequenceGen {
        let first = sequence[0];
        if sequence.iter().all(|v| *v == first) {
            return SequenceGen {
                initial_state: vec![first],
                length: sequence.len(),
            };
        }
        let delta_sequence = delta(sequence);
        let mut delta_gen = SequenceGen::from_sequence(&delta_sequence);
        delta_gen.initial_state.insert(0, first);
        delta_gen.length += 1;
        delta_gen
    }

    fn next_val(&self) -> i64 {
        self.get(self.length)
    }

    fn get(&self, i: usize) -> i64 {
        let mut state = self.initial_state.clone();
        for _ in 0..i {
            for j in 0..(state.len() - 1) {
                state[j] += state[j + 1]
            }
        }
        state[0]
    }

    fn prev(&self) -> i64 {
        let mut curr = 0;
        let len = self.initial_state.len();
        for j in (0..len).rev() {
            curr = self.initial_state[j] - curr;
        }
        curr
    }
}

pub struct SequenceGens {
    gens: Vec<SequenceGen>,
}

impl SequenceGens {
    pub fn from_lines(lines: &Vec<String>) -> Option<SequenceGens> {
        let gens = lines
            .iter()
            .map(|line| SequenceGen::from_line(line))
            .collect::<Option<Vec<SequenceGen>>>()?;
        Some(SequenceGens { gens })
    }

    pub fn next_sum(&self) -> i64 {
        self.gens.iter().map(|gen| gen.next_val()).sum()
    }

    pub fn prev_sum(&self) -> i64 {
        self.gens.iter().map(|gen| gen.prev()).sum()
    }
}
