use std::collections::{HashMap, HashSet};

struct ScratchCard {
    numbers: Vec<usize>,
    winning: HashSet<usize>,
}

pub struct ScratchCards {
    cards: Vec<ScratchCard>,
}

impl ScratchCard {
    fn from_line(line: &str) -> Option<ScratchCard> {
        let (_, nums_str) = line.split_once(": ")?;
        let (winning_str, have_str) = nums_str.split_once(" | ")?;
        let winning: HashSet<usize> = winning_str
            .split(" ")
            .filter_map(|part| part.parse::<usize>().ok())
            .collect();
        let numbers: Vec<usize> = have_str
            .split(" ")
            .filter_map(|part| part.parse::<usize>().ok())
            .collect();
        Some(ScratchCard { winning, numbers })
    }

    fn winnings(&self) -> usize {
        let overlap = self.winning_count();
        if overlap == 0 {
            return 0;
        }
        2_u64.pow((overlap - 1) as u32) as usize
    }

    fn winning_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|num| self.winning.contains(num))
            .count()
    }
}

impl ScratchCards {
    pub fn from_lines(lines: &Vec<String>) -> Option<ScratchCards> {
        let cards = lines
            .iter()
            .map(|line| ScratchCard::from_line(line))
            .collect::<Option<Vec<ScratchCard>>>()?;
        Some(ScratchCards { cards })
    }

    pub fn winnings(&self) -> usize {
        self.cards.iter().map(|card| card.winnings()).sum()
    }

    pub fn winning_scratchcards(&self) -> usize {
        let mut cached_winning: HashMap<usize, usize> = HashMap::new();
        let mut sum: usize = 0;
        for (i, _) in self.cards.iter().enumerate() {
            sum += self.winning_scratchcard(i, &mut cached_winning);
        }
        sum
    }

    fn winning_scratchcard(&self, i: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if let Some(val) = cache.get(&i) {
            return *val;
        }
        if let Some(card) = self.cards.get(i) {
            let mut sum: usize = 1;
            let overlap = card.winning_count();
            for j in 0..overlap {
                sum += self.winning_scratchcard(i + j + 1, cache);
            }
            cache.insert(i, sum);
            sum
        } else {
            0
        }
    }
}
