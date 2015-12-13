extern crate rand;
use self::rand::*;
use std::fmt;

#[derive(Debug)]
pub struct Card {
    name: String,
}

impl Card {
    pub fn generate( names: &str, colors: &str ) -> Vec<Card> {
        let mut vec = Vec::new();
        for n in names.chars() {
            for c in colors.chars() {
                vec.push( Card{name: format!("{}{}", n, c)} );
            }
        }
        thread_rng().shuffle(&mut vec[..]);
        vec
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
