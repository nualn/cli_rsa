use rsa::keys::KeyPair;

fn main() {
    // TODO: Parse args with clap
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command");
        std::process::exit(1);
    }

    let command = &args[1];

    match &command[..] {
        "generate" => {
            let keys = KeyPair::generate();
            match keys.write_to_file() {
                Ok(_) => (),
                Err(e) => panic!("Failed to write keys to file: {:?}", e),
            };
        }
        _ => println!("Unknown command {command}"),
    };
}
