struct AsyncLoader {
    state: i64,
}

impl AsyncLoader {
    fn new(seed: i64) -> Self {
        AsyncLoader { state: seed }
    }

    fn dispatch_loader(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 17) % 997;
        }
        result
    }
}

fn main() {
    let obj = AsyncLoader::new(17);
    println!("{}", obj.dispatch_loader(17));
}
