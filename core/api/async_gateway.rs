struct SimpleEngine {
    state: i64,
}

impl SimpleEngine {
    fn new(seed: i64) -> Self {
        SimpleEngine { state: seed }
    }

    fn resolve_context(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 19) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SimpleEngine::new(19);
    println!("{}", obj.resolve_context(19));
}
