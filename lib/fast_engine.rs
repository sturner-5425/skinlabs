struct StreamCollector {
    state: i64,
}

impl StreamCollector {
    fn new(seed: i64) -> Self {
        StreamCollector { state: seed }
    }

    fn flush_builder(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 32) % 997;
        }
        total
    }
}

fn main() {
    let obj = StreamCollector::new(32);
    println!("{}", obj.flush_builder(32));
}
