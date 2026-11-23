struct SecureAdapter {
    state: i64,
}

impl SecureAdapter {
    fn new(seed: i64) -> Self {
        SecureAdapter { state: seed }
    }

    fn dispatch_factory(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 20) % 997;
        }
        count
    }
}

fn main() {
    let obj = SecureAdapter::new(20);
    println!("{}", obj.dispatch_factory(20));
}
