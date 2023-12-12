use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Card {
    WildJack,
    Num(u8),
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Card {
    fn from_char(ch: char, wild_jack: bool) -> Option<Card> {
        match ch {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => {
                if wild_jack {
                    Some(Card::WildJack)
                } else {
                    Some(Card::J)
                }
            }
            'T' => Some(Card::T),
            '9' => Some(Card::Num(9)),
            '8' => Some(Card::Num(8)),
            '7' => Some(Card::Num(7)),
            '6' => Some(Card::Num(6)),
            '5' => Some(Card::Num(5)),
            '4' => Some(Card::Num(4)),
            '3' => Some(Card::Num(3)),
            '2' => Some(Card::Num(2)),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    hand_type_cache: HandType,
}

impl Hand {
    pub fn from_line(line: &str, wild_jack: bool) -> Option<Hand> {
        let (cards_str, bid_str) = line.split_once(" ")?;
        let cards_vec = cards_str
            .chars()
            .map(|ch| Card::from_char(ch, wild_jack))
            .collect::<Option<Vec<Card>>>()?;
        if cards_vec.len() != 5 {
            return None;
        }
        let cards = [
            cards_vec[0],
            cards_vec[1],
            cards_vec[2],
            cards_vec[3],
            cards_vec[4],
        ];
        let bid = bid_str.parse::<usize>().ok()?;
        let hand = Hand {
            cards,
            bid,
            hand_type_cache: HandType::FullHouse,
        };
        let hand_type_cache = hand.hand_type();
        Some(Hand {
            cards,
            bid,
            hand_type_cache,
        })
    }

    fn hand_type(&self) -> HandType {
        let mut card_counts: HashMap<Card, usize> = HashMap::new();
        for card in &self.cards {
            card_counts
                .entry(card.clone())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        if let Some(wild_size) = card_counts.remove(&Card::WildJack) {
            if let Some(&max) = card_counts.values().max() {
                for (_, count) in card_counts.iter_mut() {
                    if *count == max {
                        *count += wild_size;
                        break;
                    }
                }
            }
        }
        let counts: Vec<usize> = card_counts.values().cloned().collect();
        if counts.len() == 1 {
            return HandType::FiveOfKind;
        }
        if counts.len() == 2 {
            return if counts[0] == 4 || counts[1] == 4 {
                HandType::FourOfKind
            } else {
                HandType::FullHouse
            };
        }
        if counts.len() == 3 {
            return if counts[0] == 3 || counts[1] == 3 || counts[2] == 3 {
                HandType::ThreeOfKind
            } else {
                HandType::TwoPair
            };
        }
        if counts.len() == 4 {
            return HandType::OnePair;
        }
        HandType::HighCard
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_cmp = self.hand_type_cache.cmp(&other.hand_type_cache);
        if type_cmp != Ordering::Equal {
            type_cmp
        } else {
            self.cards.cmp(&other.cards).reverse()
        }
    }
}

pub struct Game {
    hands: Vec<Hand>,
}

impl Game {
    pub fn from_lines(lines: &Vec<String>, wild_jack: bool) -> Option<Game> {
        let mut hands = lines
            .iter()
            .map(|line| Hand::from_line(line, wild_jack))
            .collect::<Option<Vec<Hand>>>()?;
        hands.sort();
        Some(Game { hands })
    }

    pub fn winnings(&self) -> usize {
        self.hands
            .iter()
            .rev()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bid)
            .sum()
    }
}
