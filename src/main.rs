use chrono::{DateTime, Local};
use std::io::stdin;
use std::env;

struct Jot {
    short:String,
    detailed:Option<String>,
    timestamp:DateTime<Local>,
    id:i64
}

impl Jot {
    fn new(short: String, detailed: Option<String>, timestamp: DateTime<Local>) -> Self { Self { short, detailed, timestamp, id:0 } }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let timestamp = Local::now();
    let mut short = String::new();

    stdin().read_line(&mut short)
        .ok()
        .expect("Failed to get input...");

    let jot = Jot::new(short, None, timestamp);

    println!("{}\n{:?}\n{}", jot.short, jot.detailed, jot.timestamp.format("[%d-%m-%Y %H:%M:%S]"));
}
