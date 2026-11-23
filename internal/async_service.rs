struct DynamicProvider {
    state: i64,
}

impl DynamicProvider {
    fn new(seed: i64) -> Self {
        DynamicProvider { state: seed }
    }

    fn flush_monitor(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 10) % 997;
        }
        value
    }
}

fn main() {
    let obj = DynamicProvider::new(10);
    println!("{}", obj.flush_monitor(10));
}
