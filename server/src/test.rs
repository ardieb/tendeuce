use std::*;
use std;
use std::net::{TcpListener, TcpStream};
use std::sync::*;

use human::*;
use player::*;

use super::card::*;
use super::message::*;
use super::server::*;
use super::table::*;

#[test]
pub fn table_test() {
    let p1 = Arc::new(Mutex::new(Vec::new()));
    let p2 = Arc::new(Mutex::new(Vec::new()));

    let server_data = Arc::new(Mutex::new(ServerData {
        started: true,
        players: vec![
            Box::new(Human::test_new(p1.clone())),
            Box::new(Human::test_new(p2.clone())),
        ],
    }));
    let mut table = Table::new(&mut server_data.clone());

    let test1 = |msg: &str| {
        p1.lock().unwrap().push(msg.to_string());
    };
    let test2 = |msg: &str| {
        p2.lock().unwrap().push(msg.to_string());
    };

    test1("READY p1");
    test2("READY p2");
    table.wait_for_players(2);
    table.start(300, 0, Some(0)); //PLAYER p1 IS THE DEALER
    table.round();
    test1("BET 30");
    test2("BET 40");
    test1("BET 40");
    table.first_bet(10, 20);
    table.bet(3);
    table.show_card();
    table.show_card();
    table.show_card();
    test1("BET 40");
    test2("BET 40");
    table.bet(1);
    table.show_card();
    test1("BET 100");
    test2("BET 150");
    test1("BET 150");
    table.bet(1);
    table.show_card();
    test1("BET 150");
    test2("FOLD");
    table.bet(1);
    table.finalize();
}
