use operational_transform::*;

fn main() {
    let client_a = Client::new();
    let client_b = Client::new();
    let mut server = Server::new();
    let mut clients = Clients::new();
    let c_a_handle = clients.register_client(client_a);
    let c_b_handle = clients.register_client(client_b);

    // Client A makes some local changes (insert heklo at 0)
    // and does not push them
    clients.insert_at_cursor("heklo".to_string(), c_a_handle);

    // Client B makes some local changes and does not push them
    clients.insert_at_cursor("bye".to_string(), c_b_handle);

    println!("-------- CASE 1 --------");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("------- END CASE -------");

    // Client A makes more local changes (fix the typo, heklo -> hello)
    clients.move_cursor(-2, c_a_handle);
    clients.remove_at_cursor(c_a_handle);
    clients.insert_at_cursor("l".to_string(), c_a_handle);

    println!("-------- CASE 2 --------");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("------- END CASE -------");

    // Client A now pushes changes to server.
    let (c, r) = clients.push_current_changes(c_a_handle);
    // The server receives the changes (i am manual plumbing this, this would be)
    // the callback on server-side or whatever
    server.receive_changes(c, r as usize, c_a_handle, &mut clients);

    println!("-------- CASE 3 --------");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("------- END CASE -------");

    // Client A makes more changes (insert !! at end of "hello")
    clients.move_cursor(10, c_a_handle); // Move cursor to the end just, this will overflow but
    // that should be handled by the implementation
    clients.insert_at_cursor("!!".to_string(), c_a_handle);
   
    // Client A pushes the changes (this should make client B fall even more behind)
    let (c, r) = clients.push_current_changes(c_a_handle);
    server.receive_changes(c, r as usize, c_a_handle, &mut clients);

    println!("-------- CASE 4 --------");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("------- END CASE -------");

    // Client B now gets to push its changes
    let (c, r) = clients.push_current_changes(c_b_handle);
    server.receive_changes(c, r as usize, c_b_handle, &mut clients);

    println!("-------- CASE 5 --------");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("------- END CASE -------");
    // Edgecase here, assume that the client sends some changes, but before it receives an ACK,
    // the server receives something from another client, and "commits" that first

    // Client 2 removes the "bye" locally
    clients.remove_at_cursor(c_b_handle);
    clients.remove_at_cursor(c_b_handle);
    clients.remove_at_cursor(c_b_handle);

    // Push the changes to server clientside BUT DO NOT RECEIVE THEM AT SERVER ("LAG")
    let (c_inflight, r_inflight) = clients.push_current_changes(c_b_handle);

    println!("-------- CASE 6 --------");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("------- END CASE -------");

    // Again, move Client A cursor as far to the right as possible
    clients.move_cursor(10, c_a_handle);
    // Remove one of the exclamation marks 
    clients.remove_at_cursor(c_a_handle);
    // Push the changes to server and receive them
    let (c, r) = clients.push_current_changes(c_a_handle);
    server.receive_changes(c, r as usize, c_a_handle, &mut clients);

    println!("-------- CASE 7 --------");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("------- END CASE -------");

    // Receive the (late) changes from client B at server
    server.receive_changes(c_inflight, r_inflight as usize, c_b_handle, &mut clients);

    println!("------ FINAL STATE ------");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("---- END FINAL STATE ----");

    // Now for the example from the article:
    let mut server = Server::new();
    let client_a = Client::new();
    let client_b = Client::new();
    let mut clients = Clients::new();

    let client_a_handle = clients.register_client(client_a);
    let client_b_handle = clients.register_client(client_b);

    // Insert the initial string through just client
    clients.insert_at_cursor("go".to_string(), c_a_handle);
    let (c, r) = clients.push_current_changes(c_a_handle);
    server.receive_changes(c, r as usize, c_a_handle, &mut clients);

    println!("----- INITIAL STATE -----");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("--- END INITIAL STATE ---");

    // Caret of client A is already at the end of the buffer
    clients.insert_at_cursor("a".to_string(), c_a_handle);
    // this is just moving the CARET of client B to follow the example, nothing to do with the OT really
    clients.move_cursor(2, c_b_handle);
    clients.insert_at_cursor("t".to_string(), c_b_handle);

    let (changes_a, rev_a) = clients.push_current_changes(c_a_handle);
    let (changes_b, rev_b) = clients.push_current_changes(c_b_handle);

    server.receive_changes(changes_b, rev_b as usize, c_b_handle, &mut clients);
    server.receive_changes(changes_a, rev_a as usize, c_a_handle, &mut clients);


    println!("----- EX1 -----");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("--- END EX1 ---");

    // I am skipping everything until client/server asymmetry, it's a trivial example
    // already shown above
    //

    /*  The remainder does not quite work yet, i'm pushing for state keeping

    clients.insert_at_cursor("s".to_string(), c_a_handle);
    // just move to the beginning, caret will not underflow
    clients.move_cursor(-3, c_a_handle);
    let (changes_a, parent_rev_a) = clients.push_current_changes(c_a_handle);
    
    clients.remove_at_cursor(c_a_handle);

    clients.move_cursor(1, c_b_handle);
    clients.insert_at_cursor("e".to_string(), c_b_handle);
    let (changes_c, parent_rev_c) = clients.push_current_changes(c_b_handle);
    println!("{:?}", changes_c);
    let messages_c = server.receive_changes_lazy(changes_c, parent_rev_c as usize, c_b_handle, &mut clients);
    clients.insert_at_cursor("d".to_string(), c_b_handle);
    let (changes_d, parent_rev_d) = clients.push_current_changes(c_b_handle);
    println!("{:?}", changes_d);
    let messages_d = server.receive_changes_lazy(changes_d, parent_rev_d as usize, c_b_handle, &mut clients);
    
    for (i, message) in messages_c.iter().enumerate() {
        match message {
            ServerToClientMessage::Change(_,_) => { // For now skip forwarding this to client a
             }
            ServerToClientMessage::Ack(ops, rev) => {
               clients.ack(i, ops, *rev); 
            }
        }
    }
    for (i, message) in messages_d.iter().enumerate() {
        match message {
            ServerToClientMessage::Change(_,_) => { // For now skip forwarding this to client a
             }
            ServerToClientMessage::Ack(ops, rev) => {
               clients.ack(i, ops, *rev); 
            }
        }
    }

    println!("----- EX2 -----");
    println!("Client A state: {}", clients.content(c_a_handle));
    println!("Client B state: {}", clients.content(c_b_handle));
    println!("Server state: {}", server.content());
    println!("--- END EX2 ---");
    */

    

}
#[derive(Clone)]
struct Client {
    content: String,
    state: OperationSeq,
    outstanding_ops: OperationSeq,
    revision: u64,
    cursor_position: isize,
}

impl Client {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            state: OperationSeq::default(),
            outstanding_ops: OperationSeq::default(),
            revision: 0,
            cursor_position: 0,
        }
    }

    pub fn move_cursor(&mut self, delta: isize) {
        if (self.cursor_position as isize + delta) < 0 {
            self.cursor_position = 0;
        } else if (self.cursor_position as isize + delta) > self.content.len() as isize {
            self.cursor_position = self.content.len() as isize;
        } else {
            self.cursor_position += delta;
        }
    }

    pub fn insert_at_cursor(&mut self, s: String) {
        let mut operation = OperationSeq::default();
        operation.retain(self.cursor_position as u64);
        operation.insert(&s);
        operation.retain(self.content.len() as u64 - self.cursor_position as u64);
        self.content = operation.apply(&self.content).unwrap();
        self.cursor_position += s.len() as isize;
        self.outstanding_ops = self.outstanding_ops.compose(&operation).unwrap();
    }

    pub fn remove_at_cursor(&mut self) {
        let mut operation = OperationSeq::default();
        operation.retain(self.cursor_position as u64 - 1);
        operation.delete(1);
        operation.retain(self.content.len() as u64 - self.cursor_position as u64);
        self.cursor_position -= 1;
        self.content = operation.apply(&self.content).unwrap();
        self.outstanding_ops = self.outstanding_ops.compose(&operation).unwrap();
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    // Eventually this somehow sends this to server but for now
    // i'm manually plumbing things so let's see...
    pub fn push_current_changes(&mut self) -> (OperationSeq, u64) {
        let r = (self.outstanding_ops.clone(), self.revision);
        r
    }

    pub fn receive_changes(&mut self, changes: &OperationSeq, rev: u64) {
        self.state = self.state.compose(&changes).unwrap();
        let (a_p, b_p) = self.outstanding_ops.transform(&changes).unwrap();
        self.outstanding_ops = a_p;
        self.content = b_p.apply(&self.content).unwrap();
        self.revision = rev;
    }

    pub fn ack(&mut self, rev_changes: &OperationSeq, rev: usize) {
        self.outstanding_ops = OperationSeq::default();
        self.outstanding_ops.retain(self.content.len() as u64);
        self.state = rev_changes.clone();
        self.revision = rev as u64;
    }
}

struct Server {
    revisions: Vec<OperationSeq>,
    text: String,
    //clients: Vec<Client>,
}

enum ServerToClientMessage {
    Ack(OperationSeq, usize),
    Change(OperationSeq, usize)
}

impl Server {
    pub fn new() -> Self {
        Self {
            revisions: vec![],
            text: String::new(),
            //   clients: vec![],
        }
    }

    pub fn receive_changes(
        &mut self,
        mut changes: OperationSeq,
        revision: usize,
        handle: usize,
        clients: &mut Clients,
    ) {
        if revision == self.revisions.len() {
            self.text = changes.apply(&self.text).unwrap();
            for (i, c) in clients.0.iter_mut().enumerate() {
                if i != handle {
                    c.receive_changes(&changes, self.revisions.len() as u64);
                }
            }
            clients.ack(handle, &changes, self.revisions.len());
            self.revisions.push(changes);
        } else {
            // i think +1 here makes sense somehow...
            for r in self.revisions[revision+1..].iter() {

                let (a_p, _) = changes.transform(&r).unwrap();
                changes = a_p;
                //println!("OK");
                //changes.transform(&b_p).ok();
            }
            self.text = changes.apply(&self.text).unwrap();
            for (i, c) in clients.0.iter_mut().enumerate() {
                if i != handle {
                    c.receive_changes(&changes, self.revisions.len() as u64);
                }
            }
            clients.ack(handle, &changes, self.revisions.len());
            self.revisions.push(changes)
        }
    }

    // This handles the case where the Server is slow with relaying received changes to other
    // clients
    pub fn receive_changes_lazy(
        &mut self,
        mut changes: OperationSeq,
        revision: usize,
        handle: usize,
        clients: &mut Clients,
    ) ->Vec<ServerToClientMessage> {
        let mut messages = vec![];
        if revision == self.revisions.len() {
            self.text = changes.apply(&self.text).unwrap();
            for (i, _) in clients.0.iter_mut().enumerate() {
                if i != handle {
                    messages.push(ServerToClientMessage::Change(changes.clone(), self.revisions.len()));
                }
                else {
                    messages.push(ServerToClientMessage::Ack(changes.clone(), self.revisions.len()))
                }
            }
            self.revisions.push(changes);
            messages
        } else {
            // i think +1 here makes sense somehow...
            for r in self.revisions[revision+1..].iter() {

                let (a_p, _) = changes.transform(&r).unwrap();
                changes = a_p;
                //println!("OK");
                //changes.transform(&b_p).ok();
            }
            self.text = changes.apply(&self.text).unwrap();
            for (i, _) in clients.0.iter_mut().enumerate() {
                if i != handle {
                    messages.push(ServerToClientMessage::Change(changes.clone(), self.revisions.len()))
                } else {
                    messages.push(ServerToClientMessage::Ack(changes.clone(), self.revisions.len()));
                }
            }
            self.revisions.push(changes);
            messages
        }
    }

    pub fn content(&self) -> String {
        self.text.clone()
    }
}
#[derive(Clone)]
struct Clients(Vec<Client>);

impl Clients {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn register_client(&mut self, client: Client) -> usize {
        let id = self.0.len();
        self.0.push(client);
        id
    }
    pub fn move_cursor(&mut self, delta: isize, handle: usize) {
        self.0[handle].move_cursor(delta);
    }

    pub fn insert_at_cursor(&mut self, s: String, handle: usize) {
        self.0[handle].insert_at_cursor(s);
    }
    pub fn remove_at_cursor(&mut self, handle: usize) {
        self.0[handle].remove_at_cursor();
    }

    pub fn content(&self, handle: usize) -> String {
        self.0[handle].content()
    }

    // Eventually this somehow sends this to server but for now
    // i'm manually plumbing things so let's see...
    pub fn push_current_changes(&mut self, handle: usize) -> (OperationSeq, u64) {
        self.0[handle].push_current_changes()
    }

    pub fn ack(&mut self, handle: usize, changes: &OperationSeq, rev: usize) {
        self.0[handle].ack(changes, rev)
    }
}
