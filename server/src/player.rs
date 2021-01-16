use card::*;

pub trait Player {
    fn get_message(&mut self) -> Option<String>;
    fn wait_for_message(&mut self) -> String;
    fn set_cards(&mut self, cards: [Card; 2]);
    fn get_cards(&self) -> [Card; 2];
    fn get_name(&self) -> Option<String>;
    fn set_name(&mut self, name: String);
    fn get_money(&self) -> i32;
    fn set_money(&mut self, money: i32);
    fn get_fold(&self) -> bool;
    fn set_fold(&mut self, money: bool);
    fn get_bet(&self) -> i32;
    fn set_bet(&mut self, bet: i32);
    fn bet(&mut self, bet: i32);
    fn is_allin(&self) -> bool {
        self.get_money() == 0
    }
    fn is_dead(&self) -> bool;
    fn send(&mut self, &str);
}
