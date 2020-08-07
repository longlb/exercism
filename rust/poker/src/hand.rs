use super::card::{Card, Rank};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    High,
    Pair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush,
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
    StraightFlush(Rank),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    value: HandRank,
}

impl Hand {
    pub fn from(hand: &str) -> Self {
        let mut cards: Vec<Card> = hand
            .split_whitespace()
            .map(|c| Card::from(c).expect("Bad card in hand."))
            .collect();
        cards.sort();

        Self {
            value: Hand::value(&cards),
            cards,
        }
    }

    fn value(cards: &[Card]) -> HandRank {
        let straight = Hand::is_straight(cards);
        let flush = Hand::is_flush(cards);

        match (straight > 0, flush) {
            (true, true) => HandRank::StraightFlush(straight),
            (true, false) => HandRank::Straight(straight),
            (false, true) => HandRank::Flush,
            _ => Hand::dupes(cards),
        }
    }

    fn dupes(cards: &[Card]) -> HandRank {
        let dupes = cards.iter().fold(HashMap::new(), |mut acc, c| {
            let counter = acc.entry(c.rank()).or_insert(0usize);
            *counter += 1;
            acc
        });

        let quad = dupes.iter().find(|(_, &v)| v == 4usize);
        let trip = dupes.iter().find(|(_, &v)| v == 3usize);
        let pair = dupes.iter().find(|(_, &v)| v == 2usize);

        if let Some((k, _)) = quad {
            HandRank::FourOfAKind(*k)
        } else if let Some((k, _)) = trip {
            if let Some((b, _)) = pair {
                HandRank::FullHouse(*k, *b)
            } else {
                HandRank::ThreeOfAKind(*k)
            }
        } else if let Some((k, _)) = pair {
            let fake_dupes = dupes.iter().filter(|(_, &v)| v == 2usize);
            if fake_dupes.clone().count() > 1 {
                HandRank::TwoPair(
                    *fake_dupes.clone().max().unwrap().0,
                    *fake_dupes.min().unwrap().0,
                )
            } else {
                HandRank::Pair(*k)
            }
        } else {
            HandRank::High
        }
    }

    fn is_straight(cards: &[Card]) -> Rank {
        let ace_straight = cards[0].rank() == 2 && cards[4].rank() == 14;

        match match ace_straight {
            false => cards,
            true => &cards[0..4],
        }
        .windows(2)
        .all(|c| c[0].rank() + 1 == c[1].rank())
        {
            true => match ace_straight {
                true => 5,
                false => cards.iter().max().unwrap().rank(),
            },
            false => 0,
        }
    }

    fn is_flush(cards: &[Card]) -> bool {
        cards.windows(2).all(|c| c[0].suit() == c[1].suit())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value != other.value {
            true => self.value.cmp(&other.value),
            false => self.cards.iter().rev().cmp(other.cards.iter().rev()),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for card in self.cards.iter() {
            write!(f, "{}\n", card)?;
        }
        write!(f, "{:?}\n", self.value)?;
        Ok(())
    }
}
