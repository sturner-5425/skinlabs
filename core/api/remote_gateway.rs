struct BatchEngine {
    state: i64,
}

impl BatchEngine {
    fn new(seed: i64) -> Self {
        BatchEngine { state: seed }
    }

    fn render_gateway(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 11) % 997;
        }
        value
    }
}

fn main() {
    let obj = BatchEngine::new(11);
    println!("{}", obj.render_gateway(11));
}
