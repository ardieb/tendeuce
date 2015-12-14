extern crate rand;
use std::cmp::Ordering;
use self::rand::*;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Card {
    name: [char; 2],
}

impl Card {
    fn new(name: &str) -> Card{
        Card{
            name: [name.chars().nth(0).unwrap(), name.chars().nth(1).unwrap()]
        }
    }

    pub fn generate( names: &str, colors: &str ) -> Vec<Card> {
        let mut vec = Vec::new();
        for n in names.chars() {
            for c in colors.chars() {
                vec.push( Card{name: [n, c]} );
            }
        }
        thread_rng().shuffle(&mut vec[..]);
        vec
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name[0], self.name[1])
    }
}

static CARD_ORDER: &'static str = "23456789TJDKA";

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool{
        CARD_ORDER.find(self.name[0]).eq(&CARD_ORDER.find(other.name[0]))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        CARD_ORDER.find(self.name[0]).partial_cmp(&CARD_ORDER.find(other.name[0]))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering{
        CARD_ORDER.find(self.name[0]).partial_cmp(&CARD_ORDER.find(other.name[0])).unwrap()
    }
}

#[test]
fn test_card_order(){
    assert!( Card::new("2♠") == Card::new("2♠") );
    assert!( Card::new("2♠") == Card::new("2♥") );
    assert!( Card::new("2♠") != Card::new("3♠") );
    assert!( Card::new("2♠") != Card::new("3♥") );

    assert!( Card::new("5♠") > Card::new("3♥") );
    assert!( Card::new("4♠") < Card::new("7♥") );

    assert!( Card::new("4♠") <= Card::new("4♠") );
    assert!( Card::new("4♠") >= Card::new("4♠") );

    assert!( Card::new("4♠") <= Card::new("4♥") );
    assert!( Card::new("4♠") >= Card::new("4♥") );

    assert!( Card::new("K♠") == Card::new("K♠") );
    assert!( Card::new("A♠") == Card::new("A♥") );

    assert!( Card::new("9♠") < Card::new("T♠") );
    assert!( Card::new("T♠") < Card::new("J♠") );
    assert!( Card::new("J♠") < Card::new("D♠") );
    assert!( Card::new("D♠") < Card::new("K♠") );
    assert!( Card::new("K♠") < Card::new("A♠") );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType{
    CARD,
    PAIR,
    TWOPAIR,
    THREE,
    STRAIGHT,
    FLUSH,
    FULLHOUSE,
    FOUR,
    SFLUSH,
}

#[derive(Debug, Clone, Eq)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    player: usize,
}

impl Hand{
    fn test_new(ht: HandType, cards: Vec<&str>) -> Hand{
        let mut vec = Vec::new();
        for card in cards {
            vec.push(Card::new(card));
        }
        Hand{
            hand_type: ht,
            cards: vec,
            player: 0,
        }
    }
}

impl Hand {
    fn find_all( cards: &[Card] ) -> Vec<Hand> {
        unimplemented!()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool{
        self.hand_type == other.hand_type && self.cards[0] == other.cards[0]
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                self.cards[0].partial_cmp(&other.cards[0])
            }
            ord => Some(ord),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering{
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                self.cards[0].cmp(&other.cards[0])
            }
            ord => ord,
        }
    }
}

#[test]
fn test_hand_order(){
    let pair2 = Hand::test_new(HandType::PAIR, vec!["2♠", "2♥"]);
    let pair3 = Hand::test_new(HandType::PAIR, vec!["3♠", "3♥"]);
    let tri2 = Hand::test_new(HandType::THREE, vec!["2♠", "2♥", "2♥"]);

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
