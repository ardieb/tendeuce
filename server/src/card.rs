use std::cmp::Ordering;
use std::fmt;
use std::ops::Sub;

use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Card {
    name: [char; 2],
}

impl Card {
    pub fn new(name: &str) -> Card {
        Card {
            name: [name.chars().nth(0).unwrap(), name.chars().nth(1).unwrap()]
        }
    }

    pub fn generate(names: &str, suits: &str) -> Vec<Card> {
        let mut vec = Vec::new();
        for n in names.chars() {
            for c in suits.chars() {
                vec.push(Card { name: [n, c] });
            }
        }
        vec[..].shuffle(&mut thread_rng());
        vec
    }

    fn fig(&self) -> char {
        self.name[0]
    }

    fn col(&self) -> char {
        self.name[1]
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name[0], self.name[1])
    }
}

static CARD_ORDER: &'static str = "_23456789TJDKA";

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        CARD_ORDER.find(self.name[0]).eq(&CARD_ORDER.find(other.name[0]))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        CARD_ORDER.find(self.name[0]).partial_cmp(&CARD_ORDER.find(other.name[0]))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        CARD_ORDER.find(self.name[0]).partial_cmp(&CARD_ORDER.find(other.name[0])).unwrap()
    }
}

impl Sub<i32> for Card {
    type Output = Card;
    fn sub(self, offset: i32) -> Card {
        let pos = CARD_ORDER.find(self.name[0]).unwrap();
        match CARD_ORDER.chars().nth((pos as i32 - offset) as usize) {
            Some(fig) => Card {
                name: [fig, self.name[1]],
            },
            None => Card {
                name: ['_', '_'],
            },
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug, Clone, Eq)]
pub struct Hand {
    pub hand_type: HandType,
    pub cards: Vec<Card>,
    pub player: usize,
}

impl Hand {
    fn test_new(ht: HandType, cards: Vec<&str>) -> Hand {
        let mut vec = Vec::new();
        for card in cards {
            vec.push(Card::new(card));
        }
        Hand {
            hand_type: ht,
            cards: vec,
            player: 0,
        }
    }
}

impl Hand {
    pub fn find_all(player: usize, cards: &[Card]) -> Vec<Hand> {
        let mut ret: Vec<Hand> = Vec::new();

        'sflush: for i in 0..cards.len() {
            let mut vec = Vec::new();
            vec.push(cards[i]);
            while vec.len() < 5 {
                match cards.iter().find(|&&card| card == *vec.last().unwrap() - 1 && card.col() == vec.last().unwrap().col()) {
                    Some(card) => vec.push(*card),
                    None => continue 'sflush,
                }
            }
            ret.push(Hand {
                hand_type: HandType::StraightFlush,
                player,
                cards: vec,
            })
        }

        'flush: for i in 0..cards.len() {
            let mut vec: Vec<Card> = cards.iter().filter(|&&card| card.col() == cards[i].col()).cloned().collect();
            if vec.len() < 5 {
                continue;
            }
            vec.sort_by(|a, b| b.cmp(a));
            ret.push(Hand {
                hand_type: HandType::Flush,
                player,
                cards: vec,
            })
        }

        'straight: for i in 0..cards.len() {
            let mut vec = Vec::new();
            vec.push(cards[i]);
            while vec.len() < 5 {
                match cards.iter().find(|&&card| card == *vec.last().unwrap() - 1) {
                    Some(card) => vec.push(*card),
                    None => continue 'straight,
                }
            }
            ret.push(Hand {
                hand_type: HandType::Straight,
                player,
                cards: vec,
            })
        }

        'four: for i in 0..cards.len() {
            let vec: Vec<Card> = cards.iter().filter(|&&card| card == cards[i]).cloned().collect();
            if vec.len() != 4 {
                continue;
            }
            ret.push(Hand {
                hand_type: HandType::FourOfAKind,
                player,
                cards: vec,
            })
        }

        'three: for i in 0..cards.len() {
            let vec: Vec<Card> = cards.iter().filter(|&&card| card == cards[i]).cloned().collect();
            if vec.len() != 3 {
                continue;
            }
            ret.push(Hand {
                hand_type: HandType::ThreeOfAKind,
                player,
                cards: vec,
            })
        }

        'pair: for i in 0..cards.len() {
            let vec: Vec<Card> = cards.iter().filter(|&&card| card == cards[i]).cloned().collect();
            if vec.len() != 2 {
                continue;
            }
            ret.push(Hand {
                hand_type: HandType::Pair,
                player,
                cards: vec,
            })
        }

        'card: for i in 0..cards.len() {
            ret.push(Hand {
                hand_type: HandType::HighCard,
                player,
                cards: vec![cards[i]],
            })
        }

        {
            let mut fullhouses = Vec::new();
            for tri in ret.iter().filter(|hand| hand.hand_type == HandType::ThreeOfAKind) {
                for par in ret.iter().filter(|hand| hand.hand_type == HandType::Pair) {
                    let mut vec: Vec<Card> = tri.cards.iter().chain(par.cards.iter()).cloned().collect();
                    vec.sort_by(|a, b| b.cmp(a));
                    fullhouses.push(Hand {
                        hand_type: HandType::FullHouse,
                        player: player,
                        cards: vec,
                    });
                }
            }
            ret.append(&mut fullhouses);
        }

        {
            let mut two_pair = Vec::new();
            for par1 in ret.iter().filter(|hand| hand.hand_type == HandType::Pair) {
                for par2 in ret.iter().filter(|hand| hand.hand_type == HandType::Pair && *hand != par1) {
                    let mut vec: Vec<Card> = par1.cards.iter().chain(par2.cards.iter()).cloned().collect();
                    vec.sort_by(|a, b| b.cmp(a));
                    two_pair.push(Hand {
                        hand_type: HandType::TwoPair,
                        player,
                        cards: vec,
                    });
                }
            }
            ret.append(&mut two_pair);
        }

        ret.sort();
        ret.dedup();
        ret
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards[0] == other.cards[0]
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                self.cards[0].partial_cmp(&other.cards[0])
            }
            ord => Some(ord),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                self.cards[0].cmp(&other.cards[0])
            }
            ord => ord,
        }
    }
}

#[test]
fn test_card_order() {
    assert!(Card::new("2♠") == Card::new("2♠"));
    assert!(Card::new("2♠") == Card::new("2♥"));
    assert!(Card::new("2♠") != Card::new("3♠"));
    assert!(Card::new("2♠") != Card::new("3♥"));

    assert!(Card::new("5♠") > Card::new("3♥"));
    assert!(Card::new("4♠") < Card::new("7♥"));

    assert!(Card::new("4♠") <= Card::new("4♠"));
    assert!(Card::new("4♠") >= Card::new("4♠"));

    assert!(Card::new("4♠") <= Card::new("4♥"));
    assert!(Card::new("4♠") >= Card::new("4♥"));

    assert!(Card::new("K♠") == Card::new("K♠"));
    assert!(Card::new("A♠") == Card::new("A♥"));

    assert!(Card::new("9♠") < Card::new("T♠"));
    assert!(Card::new("T♠") < Card::new("J♠"));
    assert!(Card::new("J♠") < Card::new("D♠"));
    assert!(Card::new("D♠") < Card::new("K♠"));
    assert!(Card::new("K♠") < Card::new("A♠"));
}

#[test]
fn test_hand_order() {
    let pair2 = Hand::test_new(HandType::Pair, vec!["2♠", "2♥"]);
    let pair3 = Hand::test_new(HandType::Pair, vec!["3♠", "3♥"]);
    let tri2 = Hand::test_new(HandType::ThreeOfAKind, vec!["2♠", "2♥", "2♥"]);

    assert!(pair2 == pair2);
    assert!(pair2 >= pair2);
    assert!(pair2 <= pair2);

    assert!(pair2 != pair3);
    assert!(pair2 < pair3);
    assert!(pair3 > pair2);

    assert!(pair3 != tri2);
    assert!(tri2 > pair3);
    assert!(pair3 < tri2);

    assert!(pair3 != tri2);
    assert!(tri2 > pair3);
    assert!(pair3 < tri2);
}

#[test]
fn test_hand_find() {
    let sflush = Hand::find_all(0, &["Aa", "Ka", "Da", "Ja", "Ta"].iter().map(|s| Card::new(s)).collect::<Vec<Card>>()[..]);
    assert!(sflush.iter().any(|hand| hand.hand_type == HandType::StraightFlush));

    let four = Hand::find_all(0, &["Aa", "Ab", "Ac", "Ad"].iter().map(|s| Card::new(s)).collect::<Vec<Card>>()[..]);
    assert!(four.iter().any(|hand| hand.hand_type == HandType::FourOfAKind));

    let fullhouse = Hand::find_all(0, &["Aa", "Ab", "Ac", "Ka", "Kb"].iter().map(|s| Card::new(s)).collect::<Vec<Card>>()[..]);
    assert!(fullhouse.iter().any(|hand| hand.hand_type == HandType::FullHouse));

    let flush = Hand::find_all(0, &["Aa", "Ta", "6a", "5a", "4a"].iter().map(|s| Card::new(s)).collect::<Vec<Card>>()[..]);
    assert!(flush.iter().any(|hand| hand.hand_type == HandType::Flush));

    let straight = Hand::find_all(0, &["9a", "8a", "7b", "6c", "5a"].iter().map(|s| Card::new(s)).collect::<Vec<Card>>()[..]);
    assert!(straight.iter().any(|hand| hand.hand_type == HandType::Straight));

    let three = Hand::find_all(0, &["9a", "9b", "9c"].iter().map(|s| Card::new(s)).collect::<Vec<Card>>()[..]);
    assert!(three.iter().any(|hand| hand.hand_type == HandType::ThreeOfAKind));

    let two_pair = Hand::find_all(0, &["9a", "9b", "Ta", "Tb"].iter().map(|s| Card::new(s)).collect::<Vec<Card>>()[..]);
    assert!(two_pair.iter().any(|hand| hand.hand_type == HandType::TwoPair));

    let pair = Hand::find_all(0, &["9a", "9b"].iter().map(|s| Card::new(s)).collect::<Vec<Card>>()[..]);
    assert!(pair.iter().any(|hand| hand.hand_type == HandType::Pair));

    let card = Hand::find_all(0, &["Ta"].iter().map(|s| Card::new(s)).collect::<Vec<Card>>()[..]);
    assert!(card.iter().any(|hand| hand.hand_type == HandType::HighCard));
}
