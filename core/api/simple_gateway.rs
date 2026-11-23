struct BatchBuilder {
    state: i64,
}

impl BatchBuilder {
    fn new(seed: i64) -> Self {
        BatchBuilder { state: seed }
    }

    fn load_registry(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 52) % 997;
        }
        result
    }
}

fn main() {
    let obj = BatchBuilder::new(52);
    println!("{}", obj.load_registry(52));
}
