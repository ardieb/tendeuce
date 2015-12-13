use std::sync::*;
use std::thread;
use std::net::TcpStream;
use std::io::*;
use player::*;
use card::*;

pub struct Human {
    stream: TcpStream,
    msgs: Arc<Mutex<Vec<String>>>,
    name: Option<String>,
    dead: Arc<Mutex<bool>>,
    cards: Option<[Card; 2]>,
    money: i32,
    bet: i32,
}

impl Human {
    pub fn new(stream: TcpStream) -> Human {
        let stream_clone = stream.try_clone();
        let human = Human{
            stream: stream,
            msgs: Arc::new(Mutex::new(Vec::new())),
            name: None,
            dead: Arc::new(Mutex::new(false)),
            cards: None,
            money: 0,
            bet: 0,
        };
        Self::start_listening(stream_clone.unwrap(), human.msgs.clone(), human.dead.clone());
        human
    }

    fn start_listening(stream: TcpStream, msgs: Arc<Mutex<Vec<String>>>, dead: Arc<Mutex<bool>>){
        thread::spawn(move ||{
            let mut reader = BufReader::new(stream);
            loop{
                let mut line = String::new();
                match reader.read_line(&mut line){
                    Ok(bytes) if bytes > 0 => {
                        println!("> {}", line.trim());
                        msgs.lock().unwrap().push(line.trim().to_string());
                    }
                    Ok(_) => {
                        println!("> Connection closed!");
                        *dead.lock().unwrap() = true;
                        return;
                    }
                    Err(err) => {
                        println!("> {}", err);
                        *dead.lock().unwrap() = true;
                        return;
                    }
                }
            }
        });
    }
}

impl Player for Human {
    fn get_message(&mut self) -> Option<String>{
        let mut msgs = self.msgs.lock().unwrap();
        if msgs.len() < 1 {
            None
        }
        else {
            Some(msgs.remove(0))
        }
    }

    fn get_name(&self) -> Option<String>{
        self.name.clone()
    }

    fn set_name(&mut self, name: String){
        self.name = Some(name);
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

    fn is_dead(&self) -> bool{
        self.dead.lock().unwrap().clone()
    }

    fn send(&mut self, msg: &String){
        println!("{}< {}", self.name.as_ref().unwrap_or(&"_".to_string()), msg);
        let _ = self.stream.write_all(msg.as_bytes());
    }

    fn get_bet(&self) -> i32{
        self.bet
    }

    fn set_bet(&mut self, bet: i32){
        self.bet = bet;
    }

    fn bet(&mut self, bet: i32){
        self.bet = bet;
    }
}
