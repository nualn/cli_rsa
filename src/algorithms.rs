use num_bigint::{BigUint, RandBigInt};

/// Checks if n is a probable prime
pub fn miller_rabin(n: &BigUint, k: usize) -> bool {
    if *n <= BigUint::from(2usize) {
        return false;
    }

    // Find s & d so that 2^s * d = n - 1
    let mut s: BigUint = BigUint::from(0usize);
    let mut d: BigUint = n - 1usize;
    while &d % 2usize == BigUint::from(0usize) {
        d /= 2usize;
        s += 1usize;
    }

    for _ in 0..k {
        let a = rand::thread_rng().gen_biguint_range(&BigUint::from(2usize), &(n - 2usize));
        let mut x = modular_pow(&a, &d, &n);
        let mut y = BigUint::from(0usize);

        let mut m: BigUint = s.clone();
        while m > BigUint::from(0usize) {
            y = modular_pow(&x, &BigUint::from(2usize), &n);
            if y == BigUint::from(1usize) && x != BigUint::from(1usize) && x != n - 1usize {
                return false;
            }
            x = y.clone();

            m -= 1usize;
        }

        if y != BigUint::from(1usize) {
            return false;
        }
    }

    true
}

/// Returns result for base^exponent % modulus
fn modular_pow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    if *modulus == BigUint::from(1usize) {
        return BigUint::from(0usize);
    }

    let mut exponent = exponent.clone();
    let mut base = base % modulus;
    let mut result = BigUint::from(1usize);

    while exponent > 0usize.into() {
        if &exponent % 2usize == BigUint::from(1usize) {
            result = (result * &base) % modulus;
        }
        exponent >>= 1;
        base = (&base * &base) % modulus;
    }

    return result;
}
