type Message = String;
#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

struct GroundStation;
impl GroundStation {
    /// the &self indicates that groundstation.send only requires a read only ref to self.
    fn send(&self, to: &mut CubeSat, msg: Message,) {
        to.mailbox.messages.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
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
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: MailBox { messages: vec![], },
    };

    println!("t0: {:?}", sat_a);
    base.send(&mut sat_a, Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();

    println!("msg: {:?}", msg);
}
