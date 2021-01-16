use std::*;
use std::sync::*;
use rand::{Rng, thread_rng, seq::IteratorRandom};

use super::bot::*;
use super::card::*;
use super::human::*;
use super::message::*;
use super::player::*;
use super::server::*;

pub struct Table {
    server: Arc<Mutex<ServerData>>,
    bank: i32,
    shared: Vec<Card>,
    shared_visible: usize,
    max_bet: i32,
    dealer: isize,
    players: isize,
}

impl Table {
    pub fn new(server: &mut Arc<Mutex<ServerData>>) -> Table {
        Table {
            server: server.clone(),
            bank: 0,
            shared: Vec::new(),
            shared_visible: 0,
            max_bet: 0,
            dealer: 0,
            players: 0,
        }
    }

    fn unwrap_msg<T>(msg: Box<dyn Message>) -> Box<T> where T: Message {
        unsafe { Box::from_raw(Box::into_raw(msg) as *mut T) }
    }

    pub fn wait_for_players(&mut self, players: i32) {
        println!("Waiting for players( 0/{} )", players);
        let mut ready = 0;
        while ready < players as usize {
            {
                let mut data = self.server.lock().unwrap();
                for player in data.players.iter_mut() {
                    while let Some(raw_msg) = player.get_message() {
                        let msg = Message::from_str(&raw_msg);
                        match msg.get_type() {
                            MessageType::Ready => {
                                let msg = Self::unwrap_msg::<ReadyMessage>(msg);
                                if let Some(_) = player.get_name() {
                                    println!("Unexpected packet: {}", raw_msg);
                                    println!("Waiting for players( {}/{} )", ready, players);
                                } else {
                                    player.set_name(msg.name);
                                    ready += 1;
                                    println!("Waiting for players( {}/{} )", ready, players);
                                }
                            }
                            MessageType::Unknown => {
                                println!("Can't parse packet: {}", raw_msg);
                            }
                            _ => {
                                println!("Unexpected packet: {}", raw_msg);
                            }
                        }
                    }
                }
                for dead_player in data.players.iter().filter(|&player| player.is_dead()) {
                    if let Some(_) = dead_player.get_name() {
                        ready -= 1;
                    }
                    if ready != players as usize {
                        println!("Waiting for players( {}/{} )", ready, players);
                    }
                }
                data.players.retain(|player| !player.is_dead());
            }
            thread::yield_now();
        }
    }

    pub fn start(&mut self, start_money: i32, bots: i32, dealer: Option<isize>) {
        let mut server = self.server.lock().unwrap();
        server.players.retain(|player| player.get_name().is_some());
        println!("\tStarting Game!");
        for i in 0..bots {
            server.players.push(Box::new(Bot::new(i)));
        }
        let msg = Message::start(&server.players[..]);
        server.send_all(msg);
        for player in server.players.iter_mut() {
            player.set_money(start_money);
        }
        self.players = server.players.len() as isize;
        self.dealer = dealer.unwrap_or((0..self.players).choose(&mut thread_rng()).unwrap());
    }

    pub fn round(&mut self) {
        let mut server = self.server.lock().unwrap();
        let msg = Message::round(self.bank, &server.players[..]);
        server.send_all(msg);

        let mut cards = Card::generate("23456789TJDKA", "♠♥♦♣");
        println!("Players:", );
        for player in server.players.iter_mut() {
            let pcards = [cards.pop().unwrap(), cards.pop().unwrap()];
            player.send(&format!("CARDS {} {}", pcards[0], pcards[1]));
            player.set_cards(pcards);
            player.set_bet(0);
            player.set_fold(false);
            println!("{}: {} coins.", player.get_name().unwrap(), player.get_money());
        }
        self.shared = vec![cards.pop().unwrap(), cards.pop().unwrap(), cards.pop().unwrap(), cards.pop().unwrap(), cards.pop().unwrap()];
        self.shared_visible = 0;


        self.dealer = self.get_pos(self.dealer + 1);
        let dealer_name = server.players[self.dealer as usize].get_name().unwrap();
        server.send_all(format!("DEALER {}", dealer_name));
        println!("{} is a dealer.", dealer_name);
    }

    pub fn show_card(&mut self) {
        let mut server = self.server.lock().unwrap();
        server.send_all(format!("CARD {}", self.shared[self.shared_visible]));
        self.shared_visible += 1;
    }

    fn get_pos(&self, mut pos: isize) -> isize {
        while pos >= self.players {
            pos -= self.players;
        }
        while pos < 0 {
            pos += self.players;
        }
        pos
    }

    pub fn first_bet(&mut self, small: i32, big: i32) {
        let mut server = self.server.lock().unwrap();
        let mut pos = self.dealer;

        pos = self.get_pos(pos + 1);
        server.players[pos as usize].bet(small);
        let msg = format!("SBLIND {} {}", server.players[pos as usize].get_name().unwrap(), small);
        server.send_all(msg);

        pos = self.get_pos(pos + 1);
        server.players[pos as usize].bet(big);
        let msg = format!("BBLIND {} {}", server.players[pos as usize].get_name().unwrap(), big);
        server.send_all(msg);

        self.max_bet = if big > small { big } else { small };
    }

    pub fn bet(&mut self, start: isize) {
        let mut server = self.server.lock().unwrap();
        println!("\tStarting Round!");
        let mut pos = self.dealer;
        pos = self.get_pos(pos + start);
        let mut check = false;
        let mut played = 0;
        let mut can_play = 0;

        for player in server.players.iter_mut() {
            if !player.get_fold() && !player.is_allin() {
                can_play += 1;
            }
        }

        while !check && can_play > 1 {
            while server.get_player(pos).get_fold() || server.get_player(pos).is_allin() {
                pos = self.get_pos(pos + 1);
                played += 1;
            }

            let msg = format!("MOVE {}", server.get_player(pos).get_name().unwrap());
            server.send_all(msg);

            let raw_msg = server.get_player(pos).wait_for_message();
            let msg = Message::from_str(&raw_msg);
            //println!(">{}", raw_msg);
            match msg.get_type() {
                MessageType::Bet => {
                    let msg = Self::unwrap_msg::<BetMessage>(msg);
                    if msg.money < self.max_bet {
                        panic!("Bet is too small!");
                    }
                    server.get_player(pos).bet(msg.money);
                    if msg.money > self.max_bet {
                        self.max_bet = msg.money;
                    }
                    let msg = format!("BET {} {}", msg.money, server.get_player(pos).get_name().unwrap());
                    server.send_all(msg);
                }
                MessageType::Fold => {
                    server.get_player(pos).set_fold(true);
                    let msg = format!("FOLD {}", server.get_player(pos).get_name().unwrap());
                    server.send_all(msg);
                }
                MessageType::Unknown => {
                    println!("Can't parse packet: {}", raw_msg);
                }
                _ => {
                    println!("Unexpected packet: {}", raw_msg);
                }
            }

            pos = self.get_pos(pos + 1);
            played += 1;

            check = played >= self.players; // WE CAN'T CHECK BEFORE EVERYONE PLAYED

            for player in server.players.iter_mut() {
                if !player.get_fold() && !player.is_allin() && player.get_bet() != self.max_bet {
                    check = false;
                }
            }
        }
        println!("\tCheck!");
    }

    pub fn finalize(&mut self) {
        let mut server = self.server.lock().unwrap();
        for player in server.players.iter_mut() {
            self.bank += player.get_bet();
        }

        let mut hands = Vec::new();
        for (id, player) in server.players.iter_mut().enumerate() {
            let player_cards = &player.get_cards();
            let cards = player_cards.iter().chain(self.shared.iter());
            let mut player_hands: Vec<Hand> = Hand::find_all(id, &cards.cloned().collect::<Vec<Card>>());
            hands.append(&mut player_hands);
        }
        hands.sort();

        let mut best: Vec<Hand> = Vec::new();
        let mut winners: Vec<usize> = server.players.iter().enumerate().filter_map(|(id, player)| if player.get_fold() { None } else { Some(id) }).collect();
        while winners.len() > 1 {
            let hand = hands.pop();
            if hand.is_some() && (best.is_empty() || best[0] == *hand.as_ref().unwrap()) {
                best.push(hand.unwrap());
            } else {
                let mut players = Vec::new();
                for hand in best.iter() {
                    if players.iter().all(|&p| p != hand.player) {
                        players.push(hand.player);
                    }
                }
                winners.retain(|w| players.iter().any(|p| p == w));
                hands.retain(|h| winners.iter().any(|&w| h.player == w));

                if let Some(hand) = hand {
                    if winners.len() <= 1 {
                        break;
                    }
                    best.clear();
                    best.push(hand);
                } else {
                    break;
                }
            }
        }
        let mut per_player = 0;
        if winners.len() > 0 {
            per_player = self.bank / winners.len() as i32;
        }
        let mut msgs = Vec::new();
        for winner in winners {
            let player = &mut server.players[winner];
            let player_money = player.get_money();
            let player_bet = player.get_bet();
            let money;

            if player.is_allin() && per_player > player_bet * self.players as i32 {
                self.bank -= player_bet * self.players as i32;
                money = player_bet * self.players as i32;
                player.set_money(player_money + player_bet * self.players as i32);
            } else {
                self.bank -= per_player;
                money = per_player;
                player.set_money(player_money + per_player);
            }
            if let Some(hand) = best.iter().find(|h| h.player == winner) {
                println!("{} won {} because of {:?}", player.get_name().unwrap(), money, hand.hand_type);
                let msg = format!("WON {} {} {:?}", player.get_name().unwrap(), money, hand.hand_type);
                msgs.push(msg);
            } else {
                println!("{} won {}", winner, money);
                let msg = format!("WON {} {} last_standing", player.get_name().unwrap(), money);
                msgs.push(msg);
            }
        }
        for player in server.players.iter() {
            let msg = format!("ENDCARDS {} {} {}", player.get_name().unwrap(), player.get_cards()[0], player.get_cards()[1]);
            msgs.push(msg);
        }
        for msg in msgs.iter_mut().rev() {
            server.send_all(msg.clone());
        }
        println!("{} left in bank", self.bank);
    }

    pub fn end(&mut self) -> bool {
        let server = self.server.lock().unwrap();
        server.players.iter().filter(|p| p.get_money() > 0 && !p.is_dead()).count() <= 1
    }
}

#[test]
fn test_finalize() {
    let shared = vec![Card::new("Ta"), Card::new("5a"), Card::new("8a"), Card::new("3b"), Card::new("Kb")];
    let c1 = [Card::new("Tb"), Card::new("5d")];
    let c2 = [Card::new("Tc"), Card::new("4c")];

    let mut p1 = Box::new(Human::test_new(Arc::new(Mutex::new(Vec::new()))));
    let mut p2 = Box::new(Human::test_new(Arc::new(Mutex::new(Vec::new()))));

    p1.set_cards(c1);
    p1.set_money(10);
    p1.set_bet(5);
    p2.set_cards(c2);
    p2.set_money(10);
    p2.set_bet(5);

    let server_data = Arc::new(Mutex::new(ServerData {
        started: true,
        players: vec![
            p1,
            p2,
        ],
    }));
    let mut table = Table {
        server: server_data.clone(),
        bank: 300,
        shared: shared,
        shared_visible: 0,
        max_bet: 0,
        dealer: 0,
        players: 2,
    };

    table.finalize();
}
