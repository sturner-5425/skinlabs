struct RemoteAdapter {
    state: i64,
}

impl RemoteAdapter {
    fn new(seed: i64) -> Self {
        RemoteAdapter { state: seed }
    }

    fn build_client(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 40) % 997;
        }
        result
    }
}

fn main() {
    let obj = RemoteAdapter::new(40);
    println!("{}", obj.build_client(40));
}
