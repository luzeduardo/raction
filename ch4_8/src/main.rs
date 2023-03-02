#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}
#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    /// requires mutable access to itself and ownership over a message
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                // when find a message returns early with the Message wrapped in Some Option
                return Some(msg);
            }
        }
        // when no messages are found, returns None
        None
    }
    
}
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

struct GroundStation;
impl GroundStation {
    /// the &self indicates that groundstation.send only requires a read only ref to self.
    fn send(&self, mailbox: &mut Mailbox, msg: Message,) {
        // yeilds ownership of a message
        mailbox.post(msg); 
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        // calls Mailbox.deliver to receive msgs, gaining ownership of a message
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

// ownership of the to variable moves into send and to is deleted.
// fn send_ownership(to: CubeSat, msg: Message) {
//     to.mailbox.messages.push(msg);
// }

// &mut prefix allows the outer scope to retain ownership of data referred to
// by the to variable
// fn send_borrow(to: &mut CubeSat, msg: Message) {
//     to.mailbox.messages.push(msg);
// }

fn main() {
    let mut mail = Mailbox { messages: vec![] };

    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message { to: sat_id, content: String::from("hello!") };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}
