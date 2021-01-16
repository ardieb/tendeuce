use std::*;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::*;
use std::io::BufReader;

fn main() {
    print!("IP: ");
    io::stdout().flush().unwrap();
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    if ip == "\n" {ip = "127.0.0.1:9001".to_string();}
    let mut stream = TcpStream::connect(ip.trim()).unwrap();
    {
        let stream_clone = stream.try_clone().unwrap();
        thread::spawn(|| {
            let mut reader = BufReader::new(stream_clone);
            loop{
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                print!("{}", line);
            }
        });
    }
    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        stream.write_all(line.as_bytes()).unwrap();
    }
}
