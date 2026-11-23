struct BatchRegistry {
    state: i64,
}

impl BatchRegistry {
    fn new(seed: i64) -> Self {
        BatchRegistry { state: seed }
    }

    fn flush_worker(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 90) % 997;
        }
        total
    }
}

fn main() {
    let obj = BatchRegistry::new(90);
    println!("{}", obj.flush_worker(90));
}
