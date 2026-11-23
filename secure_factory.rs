struct DynamicRouter {
    state: i64,
}

impl DynamicRouter {
    fn new(seed: i64) -> Self {
        DynamicRouter { state: seed }
    }

    fn handle_gateway(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 73) % 997;
        }
        total
    }
}

fn main() {
    let obj = DynamicRouter::new(73);
    println!("{}", obj.handle_gateway(73));
}
