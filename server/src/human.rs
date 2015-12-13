use std::sync::*;
use std::thread;
use std::net::TcpStream;
use std::io::*;
use player::*;
use card::*;
use std;

pub struct Human {
    stream: Option<TcpStream>,
    msgs: Arc<Mutex<Vec<String>>>,
    name: Option<String>,
    dead: Arc<Mutex<bool>>,
    cards: Option<[Card; 2]>,
    money: i32,
    fold: bool,
    bet: i32,
}

impl Human {
    pub fn new(stream: TcpStream) -> Human {
        let stream_clone = stream.try_clone();
        let human = Human{
            stream: Some(stream),
            msgs: Arc::new(Mutex::new(Vec::new())),
            name: None,
            dead: Arc::new(Mutex::new(false)),
            cards: None,
            money: 0,
            fold: false,
            bet: 0,
        };
        Self::start_listening(stream_clone.unwrap(), human.msgs.clone(), human.dead.clone());
        human
    }

    pub fn test_new(msgs: Arc<Mutex<Vec<String>>>) -> Human {
        let human = Human{
            stream: None,
            msgs: msgs,
            name: None,
            dead: Arc::new(Mutex::new(false)),
            cards: None,
            money: 0,
            fold: false,
            bet: 0,
        };
        human
    }

    fn start_listening(stream: TcpStream, msgs: Arc<Mutex<Vec<String>>>, dead: Arc<Mutex<bool>>){
        thread::spawn(move ||{
            let mut reader = BufReader::new(stream);
            loop{
                let mut line = String::new();
                match reader.read_line(&mut line){
                    Ok(bytes) if bytes > 0 => {
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
            let msg = msgs.remove(0);
            println!("> {}", msg);
            Some(msg)
        }
    }

    fn wait_for_message(&mut self) -> String{
        loop{
            {
                let mut msgs = self.msgs.lock().unwrap();
                if msgs.len() >= 1 {
                    let msg = msgs.remove(0);
                    println!("> {}", msg);
                    return msg;
                }else if self.dead.lock().unwrap().clone() {
                    return "FOLD".to_string();
                }
            }
            thread::yield_now();
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

    fn send(&mut self, msg: &str){
        println!("{}< {}", self.name.as_ref().unwrap_or(&"_".to_string()), msg);
        if self.stream.is_none(){
            return
        }
        let _ = self.stream.as_mut().unwrap().write_all(msg.as_bytes());
        let _ = self.stream.as_mut().unwrap().write_all("\n".as_bytes());
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
