#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

extern crate rand;

use std::*;
use std::io::prelude::*;

use server::*;
use table::*;

mod server;

mod player;
mod human;
mod bot;
mod message;
mod card;
mod table;

mod test;

fn read_number(text: &str, def: i32, min: i32, max: i32) -> i32 {
    'start: loop {
        print!("{}", text);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "" {
            return def;
        }
        let num = match line.trim().parse::<i32>() {
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
    let port = read_number("Port number <default = 9001>: ", 9001, 0, u16::MAX as i32);
    let players = read_number("Players count <default = 1>: ", 1, 0, 11);
    let bots = read_number("Bots count <default = 1>: ", 1, 0, 11 - players);
    let money = read_number("Money per player <default = 300>: ", 300, 0, i32::MAX);
    let small_blind = read_number("Small blind <default = 10>: ", 10, 0, i32::MAX);
    let big_blind = read_number("Big blind <default = 20>: ", 20, 0, i32::MAX);

    let mut server = Server::start_listening(port as u16, players);
    let mut table = Table::new(&mut server);

    table.wait_for_players(players);
    table.start(money, bots, None);
    while !table.end() {
        table.round();
        table.first_bet(small_blind, big_blind);
        table.bet(3);
        table.show_card();
        table.show_card();
        table.show_card();
        table.bet(1);
        table.show_card();
        table.bet(1);
        table.show_card();
        table.bet(1);
        table.finalize();
    }

    println!("End!");
}
