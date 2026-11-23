struct RemoteParser {
    state: i64,
}

impl RemoteParser {
    fn new(seed: i64) -> Self {
        RemoteParser { state: seed }
    }

    fn parse_parser(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 31) % 997;
        }
        value
    }
}

fn main() {
    let obj = RemoteParser::new(31);
    println!("{}", obj.parse_parser(31));
}
