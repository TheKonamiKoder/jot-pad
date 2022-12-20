use std::env;
use std::fs;
use chrono::{DateTime, Local};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Jot {
    short:String,
    detailed:Option<String>,
    timestamp:String,
    id:i64
}

impl Jot {
    fn new(short: String, detailed: Option<String>, timestamp: DateTime<Local>) -> Self { Self { short, detailed, timestamp:timestamp.format("[%d-%m-%Y %H:%H:%S]").to_string(), id:0 } }
}

fn main() {
    

    let timestamp = Local::now();
    let args: Vec<String> = env::args().collect();
    
    let cmd = &args[1];

    let jot:Option<Jot> = match cmd.as_str() {
        "new" => Some(Jot::new(args[2].as_str().to_string(), None, timestamp)),
        _ => None
    };

    match jot {
        Some(jot) => println!("Timestamp: {}\nShort: {}\nDetailed: {:?}", jot.timestamp, jot.short, jot.detailed),
        None => println!("JOT NOT INITIALIZED CORRECTLY")
    }
}
