use super::descriptors::*;
use super::lookup::*;

/// Category is an enum representing the class of a poker hand.
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Copy, Clone, Debug)]
pub enum Category {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

/// Category a rank belongs to
/// # Param category: the 32 bit rank of the hand
/// # Returns: the Category enum for the given hand rank
/// A rank is a 32 bit integer 1..7462 that represents one of the
/// equivalency classes for a poker hand. In all there are 7462 such
/// equivalency classes. They are scored in order of best to worst starting
/// with the smallest value.
pub fn category(category: &u32) -> Category {
    match *category {
        1..=10 => Category::StraightFlush,
        11..=166 => Category::FourOfAKind,
        167..=322 => Category::FullHouse,
        323..=1599 => Category::Flush,
        1600..=1609 => Category::Straight,
        1610..=2463 => Category::ThreeOfAKind,
        2468..=3325 => Category::TwoPair,
        3326..=6185 => Category::Pair,
        6186..=7462 => Category::HighCard,
        _ => panic!("Cannot determine category of hand: {}", *category)
    }
}

/// A description of the rank
/// # Param category: the 32 bit rank of the hand
/// # Returns: a tuple containing a representative str of cards and a str description
pub fn description(category: &u32) -> (&'static str, &'static str) {
    DESCRIPTORS[*category as usize]
}

/// The rank id of the card
/// # Param card: the id of the card, 0..52
/// # Returns: the rank id of the card, 0..12
fn rank_from_id(card: &u32) -> u32 {
    (*card >> 2) & 0xFF
}

/// The suit id of the card
/// # Param card: the id of the card, 0..52
/// # Returns: the suit id of the card, 0..4
fn suit_from_id(card: &u32) -> u32 {
    *card & 0x3
}

/// Evaluates a hand to a rank, 1..7462
/// # Param cards: an iterator of cards
/// # Param n: the size of the hand, 5..7
/// # Returns: the rank of the hand
pub fn eval<'a>(cards: impl Iterator<Item = &'a u32>, n: usize) -> u32 {
    if n < 5 || n > 7 {
        panic!("Cannot evaluate a hand of size {}. Only 5, 6, or 7 card hands", n);
    }
    let mut sh: usize = 0;
    let mut sbin: [usize; 4] = [0; 4];
    let mut quin: [usize; 13] = [0; 13];

    for card in cards {
        let id = *card as usize;
        let suit = suit_from_id(card) as usize;
        let rank = rank_from_id(card) as usize;
        sh += SUIT_BINS[id] as usize;
        sbin[suit] |= VALUE_BINS[id] as usize;
        quin[rank] += 1;
    }

    if SUITS[sh] != 0 {
        return FLUSH[sbin[SUITS[sh] as usize - 1]] as u32;
    }

    let mut hash: usize = 0;
    let mut k: usize = n;
    for i in 0..13 {
        hash += DP[quin[i]][13 - i - 1][k] as usize;
        if k <= quin[i] {
            break;
        }
        k -= quin[i];
    }
    (match n {
        5 => NOFLUSH5[hash],
        6 => NOFLUSH6[hash],
        7 => NOFLUSH7[hash],
        _ => panic!("Failed during hash lookup!")
    }) as u32
}