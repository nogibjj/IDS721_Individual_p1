use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Parser)]
#[clap(version = "1.0", author = "Yuxin Song", about = "Time command")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yuxin Song")]
    Date {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Date { name }) => {
            let result = say_hi::say_hi(&name);
            let start = SystemTime::now();
            let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

            if result.len() < 11 {
                println!("Current time is {:?}", since_the_epoch);
            }
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}