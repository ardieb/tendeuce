
use player::*;

#[derive(Debug)]
pub enum MessageType {
    UNKNOWN,
    READY,
    BET,
    FOLD,
}

macro_rules! count_exprs {
    () => (0);
    ($head:ident $(, $tail:ident)*) => (1 + count_exprs!($($tail),*));
}

macro_rules! define_messages {
    (
        $(
            $name:ident($message_type:expr => $( $dataname:ident : $datatype:ty ),* );
        )*
    ) => {
        $(
            pub struct $name {
                $(
                    pub $dataname : $datatype,
                )*
                _nothing: (),
            }
            impl Message for $name {
                fn get_type(&self) -> MessageType{
                    return $message_type;
                }
                #[allow(unused_mut, unused_comparisons)]
                fn parse(mut vec: Vec<&str>) -> Option<$name>{
                    let _ = vec.remove(0);
                    if vec.len() < count_exprs!( $( $dataname ),* ) {
                        return None;
                    }
                    $(
                        let $dataname = match vec.remove(0).parse::<$datatype>(){
                            Ok(x) => x,
                            _ => return None,
                        };
                    )*
                    Some($name{
                        $(
                            $dataname: $dataname,
                        )*
                        _nothing: (),
                    })
                }
            }
        )*
    }
}

macro_rules! try_box {
    (
        $msg:expr, $default:ident
    ) => (
        match $msg {
            Some(msg) => Box::new(msg),
            None => Box::new($default::parse(vec!["MissingNO."]).unwrap())
        }
    )
}

pub trait Message {
    fn get_type(&self) -> MessageType;
    fn parse(Vec<&str>) -> Option<Self> where Self: Sized;
}

impl Message{
    pub fn from_str(msg: &String) -> Box<Self>{
        let args = msg.split(' ').collect::<Vec<&str>>();
        match args[0] {
            "READY" => try_box!(ReadyMessage::parse(args), UnknownMessage),
            "BET" => try_box!(BetMessage::parse(args), UnknownMessage),
            "FOLD" => try_box!(FoldMessage::parse(args), UnknownMessage),
            _ => try_box!(UnknownMessage::parse(args), UnknownMessage)
        }
    }

    pub fn start(players: &[Box<Player + Send>]) -> String{
        let mut msg = format!("START {}", players.len());
        for player in players {
            msg = format!("{} {}", msg, player.get_name().unwrap());
        }
        msg
    }

    pub fn round(bank: i32, players: &[Box<Player + Send>]) -> String{
        let mut msg = format!("ROUND {}", bank);
        for player in players {
            msg = format!("{} {}", msg, player.get_money());
        }
        msg
    }
}

define_messages!{
    UnknownMessage(MessageType::UNKNOWN => );
    ReadyMessage(MessageType::READY => name: String);
    BetMessage(MessageType::BET => money: i32);
    FoldMessage(MessageType::FOLD => money: i32);
}
