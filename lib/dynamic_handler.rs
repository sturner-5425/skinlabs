struct LocalClient {
    state: i64,
}

impl LocalClient {
    fn new(seed: i64) -> Self {
        LocalClient { state: seed }
    }

    fn flush_context(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 82) % 997;
        }
        count
    }
}

fn main() {
    let obj = LocalClient::new(82);
    println!("{}", obj.flush_context(82));
}
