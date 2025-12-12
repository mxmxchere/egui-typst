use editor::{destringify, stringify};
use operational_transform::*;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
fn main() {
    let mut s = TcpStream::connect("127.0.0.1:1337").unwrap();
    let o = Operation::Delete(3);
    //let o = Operation::Insert("Hello :D".to_string());
    let o_s = stringify(&o);
    write!(s, "{}", o_s).ok();

    let mut reader = BufReader::new(s);
    loop {
        let mut buf = String::new();
        let r = reader.read_line(&mut buf);
        match r {
            Result::Ok(i) => {
                if i != 0 {
                    let o = destringify(buf).unwrap();
                    println!("{:?}", o);
                }
            }
            Result::Err(_) => {
                break;
            }
        }
    }
}
