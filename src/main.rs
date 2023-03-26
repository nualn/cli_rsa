use num_bigint::{BigUint, RandomBits};
use rand::Rng;
use rsa::algorithms;

const KEY_SIZE: u64 = 1024;
const MR_ITERATIONS: usize = 4;

fn main() {
    // TODO: Parse args with clap
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
    let num = generate_probable_prime();
    println!("Generated prime: {num}");
}

fn generate_probable_prime() -> BigUint {
    let mut num: BigUint = rand::thread_rng().sample(RandomBits::new(KEY_SIZE));
    while !algorithms::miller_rabin(&num, MR_ITERATIONS) {
        num = rand::thread_rng().sample(RandomBits::new(KEY_SIZE));
    }
    return num;
}
