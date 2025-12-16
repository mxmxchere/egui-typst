use operational_transform::*;

fn main() {
    let client_1 = Client::new();
    let client_2 = Client::new();
    let mut server = Server::new();
    let mut clients = Clients::new();
    let c_1_handle = clients.register_client(client_1);
    let c_2_handle = clients.register_client(client_2);

    //clients.move_cursor(4, c_1_handle); // does nothing
    clients.insert_at_cursor("heklo".to_string(), c_1_handle);

    clients.insert_at_cursor("bye".to_string(), c_2_handle);
    //let (c, r) = clients.push_current_changes(c_2_handle);
    //server.receive_changes(c, r as usize, c_2_handle, &mut clients);

    println!("Client state: {}", clients.content(c_1_handle));
    clients.move_cursor(-2, c_1_handle);
    clients.remove_at_cursor(c_1_handle);
    clients.insert_at_cursor("l".to_string(), c_1_handle);
    println!("Client state: {}", clients.content(c_1_handle));
    let (c, r) = clients.push_current_changes(c_1_handle);
    server.receive_changes(c, r as usize, c_1_handle, &mut clients);
    println!("Server state: {}", server.content());
    println!("Client 2 state: {}", clients.content(c_2_handle));

    let (c, r) = clients.push_current_changes(c_2_handle);
    server.receive_changes(c, r as usize, c_2_handle, &mut clients);

    println!("Final: ");
    println!("Client 1 state: {}", clients.content(c_1_handle));
    println!("Client 2 state: {}", clients.content(c_2_handle));
    println!("Server state: {}", server.content());
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
        self.outstanding_ops = OperationSeq::default();
        // i think this is needed so we can transform these later down the line
        self.outstanding_ops.retain(self.content.len() as u64);
        r
    }

    pub fn receive_changes(&mut self, changes: &OperationSeq, rev: u64) {
        println!("receiving changes");
        //let (a_p, b_p) = self.outstanding_ops.transform(changes).unwrap();
        println!("{:?}, {:?}", self.outstanding_ops, changes);
        self.state = self.state.compose(&changes).unwrap();
        let (a_p, b_p) = self.outstanding_ops.transform(&changes).unwrap();
        println!("{:?}, {:?}", a_p, b_p);
        self.outstanding_ops = a_p;
        self.content = b_p.apply(&self.content).unwrap();
        self.revision = rev;
    }

    pub fn ack(&mut self, rev_changes: &OperationSeq, rev: usize) {
        self.state = rev_changes.clone();
        self.revision = rev as u64;
    }
}

struct Server {
    revisions: Vec<OperationSeq>,
    text: String,
    //clients: Vec<Client>,
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
            for op in &self.revisions[revision..] {
                changes = changes.transform(&op).unwrap().0;
            }
            self.text = changes.apply(&self.text).unwrap();
            for (i, c) in clients.0.iter_mut().enumerate() {
                if i != handle {
                    c.receive_changes(&changes, revision as u64 + 1);
                }
            }
            clients.ack(handle, &changes, self.revisions.len());
            self.revisions.push(changes);
        } else {
            for r in self.revisions[revision..].iter() {
                let (a_p, b_p) = r.transform(&changes).unwrap();
                changes.transform(&b_p).ok();
            }
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
