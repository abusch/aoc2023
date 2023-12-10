use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
    sync::Mutex,
};

use aoc2023::Day;
use color_eyre::{
    eyre::{bail, ContextCompat},
    Result,
};
use itertools::Itertools;
use once_cell::sync::Lazy;

inventory::submit! {
    Day::new(7, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let mut bids = input.lines().map(parse_bid).collect::<Result<Vec<_>>>()?;
    bids.sort_by_key(|bid| bid.0);
    let winnings: u64 = bids
        .into_iter()
        .enumerate()
        .map(|(idx, bid)| {
            let rank = (idx + 1) as u64;
            bid.1 * rank
        })
        .sum();
    Ok(format!("{winnings}"))
}

fn part2(input: &str) -> Result<String> {
    let mut bids = input.lines().map(parse_bid).collect::<Result<Vec<_>>>()?;
    bids.sort_by(|bid1, bid2| cmp_part2(&bid1.0, &bid2.0));
    let winnings: u64 = bids
        .into_iter()
        .enumerate()
        .map(|(idx, bid)| {
            let rank = (idx + 1) as u64;
            bid.1 * rank
        })
        .sum();
    Ok(format!("{winnings}"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    pub fn non_joker() -> [Self; 12] {
        [
            Self::C2,
            Self::C3,
            Self::C4,
            Self::C5,
            Self::C6,
            Self::C7,
            Self::C8,
            Self::C9,
            Self::T,
            Self::Q,
            Self::K,
            Self::A,
        ]
    }
}

/// Card wrapper to change ordering for part 2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part2(Card);

impl PartialOrd for Part2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}
impl Ord for Part2 {
    fn cmp(&self, b: &Self) -> Ordering {
        if self.0 == Card::J {
            if b.0 == Card::J {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if b.0 == Card::J {
            Ordering::Greater
        } else {
            self.0.cmp(&b.0)
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Default)]
pub struct Cache(Mutex<HashMap<Hand, Kind>>);

impl Cache {
    pub fn max_kind(&self, hand: Hand) -> Kind {
        let mut map = self.0.lock().unwrap();
        *(map
            .entry(hand)
            .or_insert_with_key(|hand| hand.compute_max_kind()))
    }
}

static CACHE: Lazy<Cache> = Lazy::new(Cache::default);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hand([Card; 5]);

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Self(cards)
    }

    pub fn kind(&self) -> Kind {
        let counts = self.0.into_iter().counts();

        let mut counts_list = counts.into_iter().collect::<Vec<_>>();
        counts_list.sort_by_key(|(_c, cnt)| Reverse(*cnt));

        if counts_list[0].1 == 5 {
            Kind::FiveOfAKind
        } else if counts_list[0].1 == 4 {
            Kind::FourOfAKind
        } else if counts_list[0].1 == 3 {
            if counts_list[1].1 == 2 {
                Kind::FullHouse
            } else {
                Kind::ThreeOfAKind
            }
        } else if counts_list[0].1 == 2 {
            if counts_list[1].1 == 2 {
                Kind::TwoPair
            } else {
                Kind::OnePair
            }
        } else {
            Kind::HighCard
        }
    }

    pub fn expand_jokers(&self) -> Vec<Hand> {
        if let Some(idx) = self.0.iter().position(|c| *c == Card::J) {
            Card::non_joker()
                .into_iter()
                .flat_map(|c| {
                    let mut new_hand = self.0;
                    new_hand[idx] = c;
                    let new_hand = Hand(new_hand);
                    new_hand.expand_jokers().into_iter()
                })
                .collect::<Vec<_>>()
        } else {
            vec![*self]
        }
    }

    pub fn compute_max_kind(&self) -> Kind {
        self.expand_jokers()
            .into_iter()
            .map(|h| h.kind())
            .max()
            .unwrap()
    }
    pub fn max_kind(&self) -> Kind {
        CACHE.max_kind(*self)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind() == other.kind() {
            self.0.cmp(&other.0)
        } else {
            self.kind().cmp(&other.kind())
        }
    }
}

fn cmp_part2(a: &Hand, b: &Hand) -> Ordering {
    let (kind_a, kind_b) = (a.max_kind(), b.max_kind());
    if kind_a == kind_b {
        a.0.into_iter().map(Part2).cmp(b.0.into_iter().map(Part2))
    } else {
        kind_a.cmp(&kind_b)
    }
}

pub struct Bid(Hand, u64);

pub fn parse_bid(input: &str) -> Result<Bid> {
    let (hand_s, bid_s) = input.split_once(' ').wrap_err("Invalid bid")?;

    let hand = parse_hand(hand_s)?;
    let bid = bid_s.parse::<u64>()?;

    Ok(Bid(hand, bid))
}

pub fn parse_hand(input: &str) -> Result<Hand> {
    let hand = input
        .chars()
        .map(|c| match c {
            '2' => Ok(Card::C2),
            '3' => Ok(Card::C3),
            '4' => Ok(Card::C4),
            '5' => Ok(Card::C5),
            '6' => Ok(Card::C6),
            '7' => Ok(Card::C7),
            '8' => Ok(Card::C8),
            '9' => Ok(Card::C9),
            'T' => Ok(Card::T),
            'J' => Ok(Card::J),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => bail!("Invalid card"),
        })
        .collect::<Result<Vec<Card>>>()?;
    let hand = Hand::new(hand[..].try_into()?);
    Ok(hand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hands() {
        let hand = parse_hand("32T3K").unwrap();
        assert_eq!(hand.kind(), Kind::OnePair);
        let hand = parse_hand("KK677").unwrap();
        assert_eq!(hand.kind(), Kind::TwoPair);
        let hand = parse_hand("KTJJT").unwrap();
        assert_eq!(hand.kind(), Kind::TwoPair);
        let hand = parse_hand("T55J5").unwrap();
        assert_eq!(hand.kind(), Kind::ThreeOfAKind);
        let hand = parse_hand("QQQJA").unwrap();
        assert_eq!(hand.kind(), Kind::ThreeOfAKind);
    }

    #[test]
    fn test_ordering() {
        assert!(parse_hand("33332").unwrap() > parse_hand("2AAAA").unwrap());
    }

    #[test]
    fn test_part1() {
        let res = part1(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        )
        .unwrap();
        assert_eq!(res, "6440");
    }

    #[test]
    fn test_part2() {
        let res = part2(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        )
        .unwrap();
        assert_eq!(res, "5905");
    }
}
