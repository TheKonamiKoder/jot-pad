use std::time::{SystemTime};
use std::io::stdin;

struct Jot {
    short:String,
    detailed:Option<String>,
    timestamp:SystemTime
}

impl Jot {
    fn new(short: String, detailed: Option<String>, timestamp: SystemTime) -> Self { Self { short, detailed, timestamp } }
}

fn main() {
    let timestamp = SystemTime::now();
    let mut short = String::new();

    stdin().read_line(&mut short)
        .ok()
        .expect("Failed to get input...");
    
    let jot = Jot::new(short, None, timestamp);

    println!("{}\n{:?}\n{:?}", jot.short, jot.detailed, jot.timestamp)
}
