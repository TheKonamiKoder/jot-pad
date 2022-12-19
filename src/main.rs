use std::time::{SystemTime};

struct Jot {
    short:String,
    detailed:Option<String>,
    timestamp:SystemTime
}

impl Jot {
    fn new_from_short(short:String, timestamp: SystemTime) -> Self {Self {short, detailed:None, timestamp}}
    fn new_from_short_and_detailed(short: String, detailed: Option<String>, timestamp: SystemTime) -> Self { Self { short, detailed, timestamp } }
}

fn main() {
    println!("Hello, world!");
}
