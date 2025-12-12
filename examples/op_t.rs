use std::collections::VecDeque;

use operational_transform::*;

#[derive(Clone, Copy, PartialEq)]
enum Client {
    A,
    B,
}
fn main() {
    let mut server_queue: VecDeque<(OperationSeq, Client)> = VecDeque::new();
    let server_buffer = "";
    let (mut server_seq_a, mut server_seq_b) = (OperationSeq::default(), OperationSeq::default());

    let a_buffer = "";

    let b_buffer = "";

    let mut a_seq = OperationSeq::default();
    a_seq.insert("a");
    server_queue.push_back((a_seq.clone(), Client::A));

    let mut b_seq = OperationSeq::default();
    b_seq.insert("xy");
    server_queue.push_back((b_seq.clone(), Client::B));

    a_seq.insert("58");
    server_queue.push_back((a_seq.clone(), Client::A));

    while !server_queue.is_empty() {
        let seq = server_queue.pop_front().unwrap();
        match seq.1 {
            Client::A => server_seq_a = server_seq_a.compose(&seq.0).unwrap(),
            Client::B => server_seq_b = server_seq_b.compose(&seq.0).unwrap(),
        }
    }

    let (a_p, b_p) = server_seq_a.transform(&server_seq_b).unwrap();
    server_seq_a = server_seq_a.compose(&b_p).unwrap();
    server_seq_b = server_seq_b.compose(&a_p).unwrap();
    let a_received_seq = server_seq_a.clone();
    let b_received_seq = server_seq_b.clone();
    println!("{:?}", a_received_seq);
    println!("{:?}", b_received_seq);

    let a_buffer = a_received_seq.apply(&a_buffer).unwrap();
    let b_buffer = b_received_seq.apply(&b_buffer).unwrap();
    println!("server_buffer = {}", server_buffer);
    println!("a_buffer = {}", a_buffer);
    println!("b_buffer = {}", b_buffer);
}
