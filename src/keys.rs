use std::fs::File;
use std::io::prelude::*;

use crate::algorithms;
use num_bigint::{BigInt, RandomBits};
use rand::Rng;

const KEY_SIZE: u64 = 1024;
const MR_ITERATIONS: isize = 4;
const DEFAULT_EXP: i32 = 65_537;

struct Key {
    exp: BigInt,
    modulus: BigInt,
}

impl Key {
    fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        let key_string = self.modulus.to_string() + "\n" + &self.exp.to_string();

        let mut file = File::create(path)?;
        file.write_all(&key_string.as_bytes())?;
        Ok(())
    }
}
pub struct KeyPair {
    public: Key,
    private: Key,
}

impl KeyPair {
    pub fn generate() -> KeyPair {
        loop {
            let p = generate_probable_prime();
            let q = generate_probable_prime();

            match generate_from_primes(&p, &q) {
                Ok(key_pair) => return key_pair,
                Err(_) => continue,
            }
        }
    }

    pub fn write_to_file(&self) -> std::io::Result<()> {
        self.public.write_to_file("key.public")?;
        self.private.write_to_file("key.private")?;
        Ok(())
    }
}

fn generate_from_primes(p: &BigInt, q: &BigInt) -> Result<KeyPair, &'static str> {
    let n = p * q;

    let lambda_n = lcm(&(p - BigInt::from(1)), &(q - BigInt::from(1)));
    let e = BigInt::from(DEFAULT_EXP);

    let (_, d, _) = algorithms::extended_eucledian(&e, &lambda_n);

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
    let (qcd, _, _) = algorithms::extended_eucledian(a, b);
    a * b / qcd
}
