struct LocalSession {
    state: i64,
}

impl LocalSession {
    fn new(seed: i64) -> Self {
        LocalSession { state: seed }
    }

    fn sync_handler(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 92) % 997;
        }
        acc
    }
}

fn main() {
    let obj = LocalSession::new(92);
    println!("{}", obj.sync_handler(92));
}
