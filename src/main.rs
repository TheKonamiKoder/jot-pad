use std::env;
use chrono::{Local};
use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_json;
use rand;

#[derive(Debug, Serialize, Deserialize)]
struct Jot {
    short:String,
    detailed:Option<String>,
    timestamp:String,
    id:u64
}

impl Jot {
    fn new(short: String, detailed: Option<String>, timestamp: String, id:u64) -> Self { 
        Self { 
            short, 
            detailed, 
            timestamp,
            id 
        } 
    }

    fn gen_timestamp() -> String {
        Local::now().format("%d-%m-%Y %H:%M:%S").to_string()
    }
    
    fn gen_random_id(jots:&Vec<Jot>) -> u64 {
        let mut unique = true;
        loop {
            let id = rand::thread_rng().gen::<u64>();
    
            for jot in jots.iter() {
                if jot.id == id {
                    unique = false;
                } else {
                    unique = true;
                }
            }

            if unique {
                return id;
            }
        }
    }
}

const JOTS_STORAGE_FILE:&str = "C:/Users/44773/Coding Projects/jot-pad/src/jots.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];

    let mut jots:Vec<Jot> = serde_json::from_str(
        &std::fs::read_to_string(JOTS_STORAGE_FILE).unwrap()
    ).expect("There was an error with opening the JSON file...");

    match cmd.as_str() {
        "new" => {
            jots.push(
                Jot::new(
                    args[2].clone(),
                    None,
                    Jot::gen_timestamp(),
                    Jot::gen_random_id(&jots)
                )
            );
        },
        _ => println!("{} not a valid command!", cmd)
    }

    std::fs::write(
        JOTS_STORAGE_FILE, 
        serde_json::to_string_pretty(&jots).unwrap()
    ).unwrap();

}
