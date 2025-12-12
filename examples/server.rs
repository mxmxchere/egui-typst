use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    let (tx, rx) = mpsc::channel::<Operation>();
    std::thread::spawn(move || {
        keep_state(rx);
    });
    for stream in listener.incoming() {
        println!("Accepted new connection");
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            handle_client(stream.unwrap(), tx_clone);
        });
    }
}
use editor::{destringify, stringify};
use operational_transform::*;
fn handle_client(mut stream: TcpStream, channel: Sender<Operation>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut buf = String::new();
        reader.read_line(&mut buf).ok();
        if let Some(o) = destringify(buf) {
            channel.send(o).ok();
        } else {
            break;
        }

        let o = Operation::Insert("hi".to_string());
        let back = stringify(&o);
        write!(stream, "{}", back).ok();
    }
}

fn keep_state(receiver: Receiver<Operation>) {
    let mut sequence = OperationSeq::default();
    let mut string = String::new();
    loop {
        let o = receiver.recv().unwrap();
        match o {
            Operation::Insert(s) => sequence.insert(&s),
            Operation::Retain(i) => sequence.retain(i),
            Operation::Delete(i) => sequence.delete(i),
        };
        //sequence.sequence = sequence.compose(&s).unwrap();
        print!("Old state: {}| Apply: {:?} | New state: ", string, sequence,);
        let mut string = String::new();
        string = sequence.apply(&string).unwrap();
        println!("{}", string);
    }
}
