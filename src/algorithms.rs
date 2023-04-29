use num_bigint::{BigInt, RandBigInt};

/// Checks if n is a probable prime
pub fn miller_rabin(n: &BigInt, k: isize) -> bool {
    if *n <= BigInt::from(2) {
        return false;
    }

    // Find s & d so that 2^s * d = n - 1
    let mut s: BigInt = BigInt::from(0);
    let mut d: BigInt = n - 1;
    while &d % 2 == BigInt::from(0) {
        d /= 2;
        s += 1;
    }

    for _ in 0..k {
        let a = rand::thread_rng().gen_bigint_range(&BigInt::from(2), &(n - 2));
        let mut x = modular_pow(&a, &d, &n);
        let mut y = BigInt::from(0);

        let mut m: BigInt = s.clone();
        while m > BigInt::from(0) {
            y = modular_pow(&x, &BigInt::from(2), &n);
            if y == BigInt::from(1) && x != BigInt::from(1) && x != n - 1 {
                return false;
            }
            x = y.clone();

            m -= 1;
        }

        if y != BigInt::from(1) {
            return false;
        }
    }

    true
}

/// Returns result for base^exponent % modulus
pub fn modular_pow(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
    if *modulus == BigInt::from(1) {
        return BigInt::from(0);
    }

    let mut exponent = exponent.clone();
    let mut base = base % modulus;
    let mut result = BigInt::from(1);

    while exponent > 0.into() {
        if &exponent % 2 == BigInt::from(1) {
            result = (result * &base) % modulus;
        }
        exponent >>= 1;
        base = (&base * &base) % modulus;
    }

    return result;
}

/// Returns the greatest common divisor of two numbers,
/// and the coefficients of Bézout's identity.
pub fn extended_eucledian(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let (mut old_r, mut r) = (a.clone(), b.clone());
    let (mut old_s, mut s) = (BigInt::from(1), BigInt::from(0));
    let (mut old_t, mut t) = (BigInt::from(0), BigInt::from(1));

    while &r != &BigInt::from(0) {
        let quotient = &old_r / &r;
        old_r = old_r - &quotient * &r;
        (old_r, r) = (r, old_r);
        old_s = old_s - &quotient * &s;
        (old_s, s) = (s, old_s);
        old_t = old_t - &quotient * &t;
        (old_t, t) = (t, old_t);
    }

    (old_r, old_s, old_t)
}

/// Calculates the least common multiple of two numbers.
pub fn least_common_multiple(a: &BigInt, b: &BigInt) -> BigInt {
    let (qcd, _, _) = extended_eucledian(a, b);
    a * b / qcd
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use num_bigint::BigInt;

    #[test]
    fn miller_rabin_returns_false_for_number_under_two() {
        let numbers = [BigInt::from(2), BigInt::from(1), BigInt::from(0)];
        for number in numbers {
            let res = miller_rabin(&number, 4);
            assert!(!res)
        }
    }

    #[test]
    fn miller_rabin_returns_true_for_prime() {
        let prime = BigInt::from(7919); // 1000th prime
        let res = miller_rabin(&prime, 4);
        assert!(res)
    }

    #[test]
    fn miller_rabin_returns_false_for_nonprime() {
        let nonprime = BigInt::from(7921);
        let res = miller_rabin(&nonprime, 4);
        assert!(!res)
    }

    #[test]
    fn miller_rabin_returns_true_for_large_prime() {
        // Source: https://t5k.org/curios/page.php?number_id=6496
        let prime = BigInt::from_str(
            "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000118000000080101811009000118101080000000811000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"
        ).expect("Failed to parse number");

        let res = miller_rabin(&prime, 4);
        assert!(res)
    }

    #[test]
    fn miller_rabin_returns_false_for_large_nonprime() {
        let nonprime = BigInt::from_str(
            "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000118000000080101811009000118101080000000811000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003"
        ).expect("Failed to parse number");

        let res = miller_rabin(&nonprime, 4);
        assert!(!res)
    }

    #[test]
    fn modular_pow_returns_correct_remainder() {
        let result = modular_pow(&BigInt::from(3), &BigInt::from(2), &BigInt::from(5));

        assert_eq!(BigInt::from(4), result);
    }

    #[test]
    fn modular_pow_returns_zero_for_mod_of_one() {
        let result = modular_pow(&BigInt::from(123456), &BigInt::from(5), &BigInt::from(1));

        assert_eq!(BigInt::from(0), result);
    }

    #[test]
    fn extended_euqledian_returns_correct() {
        let result = extended_eucledian(&BigInt::from(240), &BigInt::from(46));
        assert_eq!(
            (BigInt::from(2), BigInt::from(-9), BigInt::from(47)),
            result
        );
    }

    #[test]
    fn least_common_multiple_returns_correct() {
        let result = least_common_multiple(&BigInt::from(4), &BigInt::from(6));
        assert_eq!(BigInt::from(12), result);
    }
}
