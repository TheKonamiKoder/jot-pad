use chrono;                     
use serde;                      
use serde_json;                 
use rand::Rng;
use clap::{Parser, Subcommand};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
        chrono::Local::now().format("%d-%m-%Y %H:%M:%S").to_string()
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

impl std::fmt::Display for Jot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "\n{short}\n{detailed}\nJot created at: {timestamp}\nid: {id}\n",
                short=self.short,
                detailed = self.detailed.clone().unwrap_or("".to_string()),
                timestamp = self.timestamp,
                id = self.id
            ).as_ref()
        )
    }
} 

/// A simple note taker (a jotter) :D
#[derive(Parser)]
#[clap(author = "Pratyush Joshi", version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a jot to your jot pad
    Add {
        /// A quick note for the Jot - could just be a few words or two...
        short: String,

        /// A more detailed description of the jot.
        #[clap(short, long, default_value=None)]
        detailed: Option<String>
    },
    /// Deletes a Jot from your jot pad
    Del {
        /// The id of the Jot which is to be deleted. You can see the ids of the jots by running command `jot-pad log`
        #[clap(default_value=None)]
        id:Option<u64>
    },
    /// Logs all the Jots to the terminal window
    Log {}
}

const JOTS_STORAGE_FILE:&str = "jots.json";

fn main() {
    let args = Args::parse();

    let mut jots:Vec<Jot> = serde_json::from_str(
        &std::fs::read_to_string(JOTS_STORAGE_FILE)
        .unwrap()
    )
    .expect("There was an error with opening the JSON file...");

    match args.command {
        Commands::Add { short, detailed } => {
            jots.push(
                Jot::new(
                    short,
                    detailed,
                    Jot::gen_timestamp(),
                    Jot::gen_random_id(&jots)
                )
            );
            println!("Adding your new Jot...");
        },
        Commands::Del { id } => {
            match id {
                Some(id) => {
                    // The length of the vector before, to see if the id actually exists or not.
                    let tmp_len = jots.len();
                    jots.retain(|jot| jot.id != id);
                    if jots.len() == tmp_len {
                        println!("There is no jot with that name!");                        
                    } 
                    println!("Jot with id:{id} has been deleted", id=id);
                },
                None => {
                    loop {
                        let mut confirmation = String::new();
                        
                        println!("Are you sure you want to delete all of your jots? (y/n)");

                        std::io::stdin()
                            .read_line(&mut confirmation)
                            .expect("Failed to get input...");
                        
                        if confirmation.trim().to_ascii_lowercase().as_str() == "y" {
                            jots.retain(|_| false);
                            println!("Deleted all Jots.");
                            break;
                        } else if confirmation.trim().to_ascii_lowercase().as_str() == "n"  {
                            println!("Phew... that was a close one :)");
                            break;
                        } else {
                            println!("Sorry - didn't understand the input...")
                        }
                    }
                }
            }
        },
        Commands::Log {} => {
            for jot in jots.iter() {
                print!("{}", jot);
            }
        },
    }

    std::fs::write(
        JOTS_STORAGE_FILE, 
        serde_json::to_string_pretty(&jots).unwrap()
    ).unwrap();

}
