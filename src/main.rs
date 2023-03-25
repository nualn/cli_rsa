fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command");
        std::process::exit(1);
    }

    let command = &args[1];

    match &command[..] {
        "generate" => generate_keys(),
        _ => println!("Unknown command {command}"),
    };
}

fn generate_keys() {
    println!("Generating keys");
}
