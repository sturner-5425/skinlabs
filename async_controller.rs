struct SmartCache {
    state: i64,
}

impl SmartCache {
    fn new(seed: i64) -> Self {
        SmartCache { state: seed }
    }

    fn dispatch_provider(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 85) % 997;
        }
        result
    }
}

fn main() {
    let obj = SmartCache::new(85);
    println!("{}", obj.dispatch_provider(85));
}
