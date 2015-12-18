use std::sync::*;
use std::thread;
use std::net::TcpStream;
use std::io::*;
use player::*;
use card::*;
use std;

pub struct Bot {
    name: String,
    cards: Option<[Card; 2]>,
    money: i32,
    fold: bool,
    bet: i32,
    shared: Vec<Card>,
    max_bet: i32,
}

impl Bot {
    pub fn new(nr: i32) -> Bot {
        let bot = Bot{
            name: format!("BOT{}", nr),
            cards: None,
            money: 0,
            fold: false,
            bet: 0,
            shared: Vec::new(),
            max_bet: 0,
        };
        bot
    }
}

impl Player for Bot {
    fn get_message(&mut self) -> Option<String>{
        Some(format!("READY {}", self.name))
    }

    fn wait_for_message(&mut self) -> String{
        if self.shared.len() == 0 {
            format!("BET {}", self.max_bet)
        }else{
            let cards: Vec<Card> = self.cards.unwrap().iter().chain(self.shared.iter()).cloned().collect();
            let hands = Hand::find_all(0, &cards[..]);
            let total_money = (self.money + self.bet) as f32;
            let max = match hands[0].hand_type {
                HandType::CARD => total_money * 0.3,
                HandType::PAIR => total_money * 0.4,
                HandType::TWOPAIR => total_money * 0.6,
                HandType::THREE => total_money * 0.6,
                HandType::STRAIGHT => total_money * 0.8,
                HandType::FLUSH => total_money * 0.9,
                HandType::FULLHOUSE => total_money * 0.9,
                HandType::FOUR => total_money * 1.0,
                HandType::SFLUSH => total_money * 1.0,
            };
            let max = max as i32;

            if self.max_bet > max{
                "FOLD".to_string()
            }else{
                format!("BET {}", max)
            }
        }
    }

    fn get_name(&self) -> Option<String>{
        Some(self.name.clone())
    }

    fn set_name(&mut self, name: String){
        self.name = name;
    }

    fn get_money(&self) -> i32{
        self.money
    }

    fn set_money(&mut self, money: i32){
        self.money = money;
    }

    fn set_cards(&mut self, cards: [Card; 2]){
        self.cards = Some(cards);
    }

    fn get_cards(&self) -> [Card; 2]{
        self.cards.unwrap()
    }

    fn is_dead(&self) -> bool{
        false
    }

    fn send(&mut self, msg: &str){
        //println!("<{}", msg);

        let msg: Vec<&str> = msg.split(char::is_whitespace).collect();

        match msg[0] {
            "CARDS" => self.shared.clear(),
            "CARD" => self.shared.push(Card::new(msg[1])),
            "SBLIND" | "BBLIND" => self.max_bet = msg[2].parse().unwrap(),
            "BET" => self.max_bet = msg[1].parse().unwrap(),
            _ => {}
        }
    }

    fn get_fold(&self) -> bool{
        self.fold
    }

    fn set_fold(&mut self, fold: bool){
        self.fold = fold;
    }

    fn get_bet(&self) -> i32{
        self.bet
    }

    fn set_bet(&mut self, bet: i32){
        self.bet = bet;
    }

    fn bet(&mut self, bet: i32){
        if bet - self.bet > self.money {
            self.money = 0;
        }else{
            self.money -= bet - self.bet;
            self.bet = bet;
        }
    }
}
