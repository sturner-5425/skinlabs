struct BatchRegistry {
    state: i64,
}

impl BatchRegistry {
    fn new(seed: i64) -> Self {
        BatchRegistry { state: seed }
    }

    fn resolve_handler(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 57) % 997;
        }
        value
    }
}

fn main() {
    let obj = BatchRegistry::new(57);
    println!("{}", obj.resolve_handler(57));
}
