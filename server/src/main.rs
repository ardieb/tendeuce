use std::*;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn read_number(def: i32, min: i32, max: i32) -> i32{
    'start: loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "" {
            return def;
        }
        let num = match line.trim().parse::<i32>(){
            Ok(num) if num >= min && num < max => num,
            _ => {
                print!("Try again: ");
                io::stdout().flush().unwrap();
                continue 'start;
            }
        };
        return num;
    }
}

fn main() {

    print!("Port number[9001]: ");
    io::stdout().flush().unwrap();
    let port = read_number(9001, 0, std::u16::MAX as i32);

    print!("Players count[2]: ");
    io::stdout().flush().unwrap();
    let players = read_number(2, 0, 11);

    print!("Bots count[0]: ");
    io::stdout().flush().unwrap();
    let bots = read_number(0, 0, 11-players);

    print!("Money per player[300]: ");
    io::stdout().flush().unwrap();
    let money = read_number(300, 0, std::i32::MAX);

    print!("Small blind[10]: ");
    io::stdout().flush().unwrap();
    let small_blind = read_number(10, 0, std::i32::MAX);

    print!("Big blind[10]: ");
    io::stdout().flush().unwrap();
    let big_blind = read_number(10, 0, std::i32::MAX);



    loop{

    }
}
