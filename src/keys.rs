use std::fs::File;
use std::io::{self, prelude::*, Error};

use crate::algorithms::{self, modular_pow};
use num_bigint::{BigInt, RandomBits};
use rand::Rng;
use std::str::FromStr;

const KEY_SIZE: u64 = 1024;
const MR_ITERATIONS: isize = 4;
const DEFAULT_EXP: i32 = 65_537;

#[derive(Debug)]
pub struct Key {
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

    pub fn from_file(path: &str) -> Result<Key, Error> {
        let file = File::open(path)?;
        let mut file_buf = io::BufReader::new(file).lines();

        let modulus = match file_buf.next() {
            Some(string) => string?,
            None => panic!("Invalid key file: {}", path),
        };
        let exp = match file_buf.next() {
            Some(string) => string?,
            None => panic!("Invalid key file: {}", path),
        };

        Ok(Key {
            modulus: match BigInt::from_str(&modulus) {
                Ok(num) => num,
                Err(_) => panic!("Invalid key file: {}", path),
            },
            exp: match BigInt::from_str(&exp) {
                Ok(num) => num,
                Err(_) => panic!("Invalid key file: {}", path),
            },
        })
    }
    pub fn encrypt(&self, in_path: &str, out_path: &str) -> std::io::Result<()> {
        let in_bytes: usize = (self.modulus.bits() / 8).try_into().unwrap();
        let out_bytes: usize = ((self.modulus.bits() + 7) / 8).try_into().unwrap();
        self.dencrypt(in_path, out_path, in_bytes, out_bytes)
    }

    pub fn decrypt(&self, in_path: &str, out_path: &str) -> std::io::Result<()> {
        let in_bytes: usize = ((self.modulus.bits() + 7) / 8).try_into().unwrap();
        let out_bytes: usize = ((self.modulus.bits() - 1) / 8).try_into().unwrap();
        self.dencrypt(in_path, out_path, in_bytes, out_bytes)
    }

    fn dencrypt(
        &self,
        in_path: &str,
        out_path: &str,
        in_bytes: usize,
        out_bytes: usize,
    ) -> std::io::Result<()> {
        let mut in_file = File::open(in_path)?;
        let mut out_file = File::create(out_path)?;

        let mut current_in_bytes: Vec<u8> = vec![0u8; in_bytes];

        let mut amount_of_bytes_read = in_bytes;

        while amount_of_bytes_read > 0 {
            current_in_bytes.fill(0);
            amount_of_bytes_read = in_file.read(&mut current_in_bytes)?;
            if amount_of_bytes_read == 0 {
                break;
            }

            let mut dencrypted_bytes = modular_pow(
                &BigInt::from_bytes_le(num_bigint::Sign::Plus, &current_in_bytes),
                &self.exp,
                &self.modulus,
            )
            .to_bytes_le()
            .1;

            // Fill in missing bytes
            let mut i = 0;
            while i < out_bytes - dencrypted_bytes.len() {
                dencrypted_bytes.push(0u8);
                i += 1;
            }

            out_file.write(&dencrypted_bytes)?;
        }

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
