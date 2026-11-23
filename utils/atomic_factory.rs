struct SmartResolver {
    state: i64,
}

impl SmartResolver {
    fn new(seed: i64) -> Self {
        SmartResolver { state: seed }
    }

    fn encode_registry(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 8) % 997;
        }
        result
    }
}

fn main() {
    let obj = SmartResolver::new(8);
    println!("{}", obj.encode_registry(8));
}
