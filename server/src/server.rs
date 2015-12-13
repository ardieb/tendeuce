use std::sync::*;
use std::thread;
use std::net::{TcpListener, TcpStream};
use player::*;
use human::*;
use std;

pub struct ServerData {
    pub players: Vec<Box<Player + Send>>,
    pub started: bool,
}

pub struct Server {
    data: Arc<Mutex<ServerData>>,
}

impl Server {
    pub fn start_listening(port: u16, max_connections: i32) -> Arc<Mutex<ServerData>>{
        let listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
        let server = Self::new();
        let server_data = server.data.clone();
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let players = server.data.lock().unwrap().players.len();
                        let started = server.data.lock().unwrap().started;
                        if players < max_connections as usize && !started {
                            server.handle_client(stream);
                        }
                    },
                    Err(_) => {
                        return;
                    }
                };
            };
        });
        server_data
    }

    fn new() -> Server{
        Server{
            data: Arc::new(Mutex::new(ServerData{
                players: Vec::new(),
                started: false,
            })),
        }
    }

    fn handle_client(&self, stream: TcpStream){
        let new_human = Human::new(stream);
        self.data.lock().unwrap().players.push(Box::new(new_human));
    }
}

impl ServerData {
    pub fn send_all(&mut self, msg: String){
        for player in self.players.iter_mut() {
            player.send(&msg);
        }
    }

    pub fn get_player(&mut self, mut pos: isize) -> &mut Box<Player + Send>{
        while pos >= self.players.len() as isize {
            pos -= self.players.len() as isize;
        }
        while pos < 0 {
            pos += self.players.len() as isize;
        }
        &mut self.players[pos as usize]
    }
}
