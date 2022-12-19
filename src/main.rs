use std::time::{SystemTime};

struct Jot {
    short:String,
    detailed:Option<String>,
    timestamp:SystemTime
}

impl Jot {
    fn new(short: String, detailed: Option<String>, timestamp: SystemTime) -> Self { Self { short, detailed, timestamp } }
}

fn main() {
    println!("Hello, world!");
}
