use clap::{Parser, Subcommand};
use rsa::keys::KeyPair;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate => {
            let keys = KeyPair::generate();
            match keys.write_to_file() {
                Ok(_) => (),
                Err(e) => panic!("Failed to write keys to file: {:?}", e),
            };
        }
    };
}
