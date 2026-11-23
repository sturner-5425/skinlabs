struct LiteDispatcher {
    state: i64,
}

impl LiteDispatcher {
    fn new(seed: i64) -> Self {
        LiteDispatcher { state: seed }
    }

    fn run_parser(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 72) % 997;
        }
        result
    }
}

fn main() {
    let obj = LiteDispatcher::new(72);
    println!("{}", obj.run_parser(72));
}
