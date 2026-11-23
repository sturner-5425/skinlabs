struct BatchRouter {
    state: i64,
}

impl BatchRouter {
    fn new(seed: i64) -> Self {
        BatchRouter { state: seed }
    }

    fn fetch_factory(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 17) % 997;
        }
        count
    }
}

fn main() {
    let obj = BatchRouter::new(17);
    println!("{}", obj.fetch_factory(17));
}
