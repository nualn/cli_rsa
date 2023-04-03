use num_bigint::{BigInt, RandomBits};
use rand::Rng;
use rsa::algorithms::{self, extended_eucledian};

const KEY_SIZE: u64 = 1024;
const MR_ITERATIONS: isize = 4;
const DEFAULT_E: i32 = 65_537;

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
            keys.write_to_file();
        }
        _ => println!("Unknown command {command}"),
    };
}

struct Key {
    exp: BigInt,
    modulus: BigInt,
}
struct KeyPair {
    public: Key,
    private: Key,
}

impl KeyPair {
    fn generate() -> KeyPair {
        loop {
            let p = generate_probable_prime();
            let q = generate_probable_prime();

            match generate_from_primes(&p, &q) {
                Ok(key_pair) => return key_pair,
                Err(_) => continue,
            }
        }
    }

    pub fn write_to_file(&self) {
        // TODO: write keys to file
        println!("Public Key:");
        println!("Exp: {}, mod: {}", self.public.exp, self.public.modulus);
        println!("Private Key:");
        println!("Exp: {}, mod: {}", self.private.exp, self.private.modulus);
    }
}

fn generate_from_primes(p: &BigInt, q: &BigInt) -> Result<KeyPair, &'static str> {
    let n = p * q;

    let lambda_n = lcm(&(p - BigInt::from(1)), &(q - BigInt::from(1)));
    let e = BigInt::from(DEFAULT_E);

    let (_, d, _) = extended_eucledian(&e, &lambda_n);

    if d < BigInt::from(0) {
        Err("Failed to generate private key")
    } else {
        Ok(KeyPair {
            public: Key {
                exp: e,
                modulus: n.clone(),
            },
            private: Key { exp: d, modulus: n },
        })
    }
}

fn generate_probable_prime() -> BigInt {
    let mut num: BigInt = rand::thread_rng().sample(RandomBits::new(KEY_SIZE));
    while !algorithms::miller_rabin(&num, MR_ITERATIONS) {
        num = rand::thread_rng().sample(RandomBits::new(KEY_SIZE));
    }
    return num;
}

fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
    let (qcd, _, _) = extended_eucledian(a, b);
    a * b / qcd
}
