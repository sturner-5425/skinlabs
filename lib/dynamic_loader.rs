struct CoreScheduler {
    state: i64,
}

impl CoreScheduler {
    fn new(seed: i64) -> Self {
        CoreScheduler { state: seed }
    }

    fn dispatch_session(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 60) % 997;
        }
        total
    }
}

fn main() {
    let obj = CoreScheduler::new(60);
    println!("{}", obj.dispatch_session(60));
}
