use clap::Parser;
use rand::Rng;

#[derive(Parser)]
#[clap(version = "1.0", author = "ChatGPT", about = "Roll the dice and return a random number between 1 and 6")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "ChatGPT", about = "Roll the dice and return a random number between 1 and 6")]
    RollDice {},
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::RollDice {}) => {
            let result = rand::thread_rng().gen_range(1, 7);
            println!("You rolled: {}", result);
            if result == 6 {
                println!("Congratulations! You got the biggest one!");
            }
        },
        _ => println!("Invalid command"),
    }
}
