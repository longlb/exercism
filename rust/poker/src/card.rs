use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

// Alias a type rather than make an enum from scratch
pub type Rank = u8;

// Possible Card Errors
#[derive(Debug)]
pub enum Error {
    UnknownRank,
    UnknownSuit,
}

// Suit Implementation
#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Suit::Diamond => write!(f, "Diamond"),
            Suit::Club => write!(f, "Club"),
            Suit::Heart => write!(f, "Heart"),
            Suit::Spade => write!(f, "Spade"),
        }
    }
}

// Card Implementation
#[derive(Debug)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn from(card: &str) -> Result<Self, Error> {
        let mut card = card.chars();
        Ok(Self {
            rank: Card::match_rank(card.next().unwrap())?,
            suit: Card::match_suit(card.last().unwrap())?,
        })
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    fn match_rank(c: char) -> Result<Rank, Error> {
        match c {
            '2'..='9' => Ok(c.to_digit(10).unwrap() as Rank),
            '1' => Ok(10),
            'J' => Ok(11),
            'Q' => Ok(12),
            'K' => Ok(13),
            'A' => Ok(14),
            _ => Err(Error::UnknownRank),
        }
    }

    fn match_suit(c: char) -> Result<Suit, Error> {
        match c {
            'D' => Ok(Suit::Diamond),
            'C' => Ok(Suit::Club),
            'H' => Ok(Suit::Heart),
            'S' => Ok(Suit::Spade),
            _ => Err(Error::UnknownSuit),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}s", self.rank, self.suit)
    }
}
