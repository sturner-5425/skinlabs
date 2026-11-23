struct RemoteManager {
    state: i64,
}

impl RemoteManager {
    fn new(seed: i64) -> Self {
        RemoteManager { state: seed }
    }

    fn decode_scheduler(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 24) % 997;
        }
        count
    }
}

fn main() {
    let obj = RemoteManager::new(24);
    println!("{}", obj.decode_scheduler(24));
}
