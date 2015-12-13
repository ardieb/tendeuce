extern crate rand;
use self::rand::{thread_rng, Rng};
use std::sync::*;
use std::*;
use super::server::*;
use super::message::*;
use super::card::*;

pub struct Table {
    server: Arc<Mutex<ServerData>>,
    bank: i32,
    shared: Vec<Card>,
    max_bet: i32,
    dealer: isize,
    players: isize,
}

impl Table {
    pub fn new(server: &mut Arc<Mutex<ServerData>>) -> Table {
        Table{
            server: server.clone(),
            bank: 0,
            shared: Vec::new(),
            max_bet: 0,
            dealer: 0,
            players: 0,
        }
    }

    fn unwrap_msg<T>(msg: Box<Message>) -> Box<T> where T: Message{
        unsafe{ Box::from_raw( Box::into_raw(msg) as *mut T ) }
    }

    pub fn wait_for_players(&mut self, players: i32){
        println!("Waiting for players( 0/{} )", players);
        let mut ready = 0;
        while ready < players as usize {
            {
                let mut data = self.server.lock().unwrap();
                for player in data.players.iter_mut() {
                    while let Some(raw_msg) = player.get_message() {
                        let msg = Message::from_str(&raw_msg);
                        match msg.get_type() {
                            MessageType::READY => {
                                let msg = Self::unwrap_msg::<ReadyMessage>(msg);
                                if let Some(_) = player.get_name(){
                                    println!("Unexpected packet: {}", raw_msg);
                                    println!("Waiting for players( {}/{} )", ready, players);
                                }else{
                                    player.set_name(msg.name);
                                    ready += 1;
                                    println!("Waiting for players( {}/{} )", ready, players);
                                }
                            },
                            MessageType::UNKNOWN => {
                                println!("Can't parse packet: {}", raw_msg);
                            },
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

    pub fn start(&mut self, start_money: i32, _bots: i32) {
        let mut server = self.server.lock().unwrap();
        server.players.retain(|player| player.get_name().is_some());
        println!("Starting Game!");
        let msg = Message::start(&server.players[..]);
        server.send_all(msg);
        for player in server.players.iter_mut() {
            player.set_money(start_money);
        }
        self.players = server.players.len() as isize;
        self.dealer = thread_rng().gen_range(0, self.players);
    }

    pub fn round(&mut self) {
        let mut server = self.server.lock().unwrap();
        let msg = Message::round(self.bank, &server.players[..]);
        server.send_all(msg);

        let mut cards = Card::generate("23456789TJDKA", "♠♥♦♣");
        self.shared = vec![cards.pop().unwrap(), cards.pop().unwrap(), cards.pop().unwrap(), cards.pop().unwrap(), cards.pop().unwrap()];
        for player in server.players.iter_mut() {
            let pcards = [cards.pop().unwrap(), cards.pop().unwrap()];
            player.send(&format!("CARDS {} {}", pcards[0], pcards[1]));
            player.set_cards(pcards);
            player.set_bet(0);
            player.set_fold(false);
        }

        self.dealer = self.get_pos(self.dealer + 1);;
        server.send_all(format!("DEALER {}", self.dealer))
    }

    pub fn show_card(&mut self) {
        unimplemented!()
    }

    fn get_pos(&self, mut pos: isize) -> isize{
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

        self.max_bet = if big > small {big} else {small};
    }

    pub fn bet(&mut self, start: isize) {
        let mut server = self.server.lock().unwrap();
        println!("Starting Round!");
        let mut pos = self.dealer;
        pos = self.get_pos(pos + start);
        let mut check = false;
        while !check {
            let msg = format!("MOVE {}", server.get_player(pos).get_name().unwrap());
            server.send_all(msg);
            let raw_msg = server.get_player(pos).wait_for_message();
            let msg = Message::from_str(&raw_msg);
            match msg.get_type() {
                MessageType::BET => {
                    let msg = Self::unwrap_msg::<BetMessage>(msg);
                    server.get_player(pos).bet(msg.money);
                    if msg.money > self.max_bet {
                        self.max_bet = msg.money;
                    }
                },
                MessageType::FOLD => {
                    server.get_player(pos).set_fold(true);
                },
                MessageType::UNKNOWN => {
                    println!("Can't parse packet: {}", raw_msg);
                },
                _ => {
                    println!("Unexpected packet: {}", raw_msg);
                }
            }
            check = true;

            for player in server.players.iter_mut() {
                if !player.get_fold() && !player.is_allin() && player.get_bet() != self.max_bet {
                    check = false;
                }
            }
        }
        println!("Check!");
    }

    pub fn finalize(&mut self) {

    }

    pub fn end(&mut self) -> bool {
        false
    }
}
