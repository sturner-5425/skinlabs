struct FastGateway {
    state: i64,
}

impl FastGateway {
    fn new(seed: i64) -> Self {
        FastGateway { state: seed }
    }

    fn collect_provider(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 51) % 997;
        }
        result
    }
}

fn main() {
    let obj = FastGateway::new(51);
    println!("{}", obj.collect_provider(51));
}
