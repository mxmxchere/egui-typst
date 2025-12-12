//use kyte::Compose;
use operational_transform::*;

fn main() {
    let mut server_sequence = OperationSeq::default();
    let mut server_string;

    let mut alice_sequence = OperationSeq::default();
    let mut alice_string = String::new();

    let mut bob_sequence = OperationSeq::default();
    let mut bob_string = String::new();

    alice_sequence.insert("Hi");

    bob_sequence.insert("Hello");

    server_sequence = if let Ok(s) = server_sequence.compose(&alice_sequence) {
        bob_sequence.compose(&alice_sequence).ok();
        s
    } else {
        let (a_p, b_p) = server_sequence.transform(&alice_sequence).unwrap();
        bob_sequence = bob_sequence.compose(&b_p).unwrap();
        alice_sequence = alice_sequence.compose(&a_p).unwrap();
        server_sequence.compose(&b_p).unwrap()
    };

    server_string = server_sequence.apply(&String::new()).unwrap();

    server_sequence = if let Ok(s) = server_sequence.compose(&bob_sequence) {
        alice_sequence.compose(&bob_sequence);
        s
    } else {
        // one would send a_p back to Bob
        // and send b_p also onto Alice
        let (a_p, b_p) = server_sequence.transform(&bob_sequence).unwrap();
        bob_sequence = bob_sequence.compose(&a_p).unwrap();
        alice_sequence = alice_sequence.compose(&b_p).unwrap();
        server_sequence.compose(&b_p).unwrap()
    };
    server_string = server_sequence.apply(&String::new()).unwrap();
    alice_string = alice_sequence.apply(&String::new()).unwrap();
    bob_string = bob_sequence.apply(&String::new()).unwrap();
    println!("server_string: {}", server_string);
    println!("Alice string: {}", alice_string);
    println!("Bob string: {}", bob_string);

    bob_sequence.delete(1);

    server_sequence = if let Ok(s) = server_sequence.compose(&bob_sequence) {
        alice_sequence.compose(&bob_sequence);
        s
    } else {
        // one would send a_p back to Bob
        // and send b_p also onto Alice
        let (a_p, b_p) = server_sequence.transform(&bob_sequence).unwrap();
        bob_sequence = bob_sequence.compose(&a_p).unwrap();
        alice_sequence = alice_sequence.compose(&b_p).unwrap();
        server_sequence.compose(&b_p).unwrap()
    };
    server_string = server_sequence.apply(&String::new()).unwrap();
    alice_string = alice_sequence.apply(&String::new()).unwrap();
    bob_string = bob_sequence.apply(&String::new()).unwrap();
    println!("server_string: {}", server_string);
    println!("Alice string: {}", alice_string);
    println!("Bob string: {}", bob_string);
}
