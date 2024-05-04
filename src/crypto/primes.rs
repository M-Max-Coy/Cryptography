use num_bigint::BigInt;
use crate::crypto::math_helper;

pub fn random_binary_vec(bits: usize) -> Vec<u8> {
    let mut bit_array: Vec<u8> = vec![0; bits];

    for i in 0..bits {
        let cur_bit = rand::random::<bool>();
        bit_array[i] = match cur_bit {
            true => 1,
            false => 0,
        }
    }

    return bit_array
}

pub fn get_prime(bits: usize) -> BigInt {
    let mut binary: Vec<u8> = random_binary_vec(bits);
    binary[0] = 1; // Always greater than or equal to 2^(bits-1)
    binary[bits-1] = 1; // Always odd
    let binary_str: String = binary.iter().map(|&bit| char::from(bit + b'0')).collect();
    let mut n: BigInt = BigInt::parse_bytes(binary_str.as_bytes(), 2).unwrap();

    let mut bases: Vec<BigInt> = Vec::new();
    bases.push(BigInt::from(2)); bases.push(BigInt::from(3)); bases.push(BigInt::from(5)); bases.push(BigInt::from(7)); bases.push(BigInt::from(11)); bases.push(BigInt::from(13)); bases.push(BigInt::from(17));
    let ub = (&n << 1) + 1;
    while n < ub {
        if is_prime_prob(&n, &bases[..]){
            break;
        }

        n = n + 2;
    }

    return n;
}

/*
Checks whether a is a witness for the compositeness of n

Parameters:
    a - witness to test
    n - number to test compositeness of
    k - pre computed value for n=2^k*q+1
    q - pre computed value for n=2^k*q+1
*/
fn miller_rabin_test(a: &BigInt, n: &BigInt, k: &BigInt, q: &BigInt) -> bool {
    // Preliminary checks
    if n & BigInt::from(1) == BigInt::from(0) {
        return true;
    }
    let gcd = math_helper::gcd(&a,&n);
    if gcd > BigInt::from(1) && &gcd < n {
        return true;
    }
    
    // Start miller rabin test
    let mut a0: BigInt = math_helper::fast_power(a,&q,n);
    if a0 == BigInt::from(1) {
        return false;
    }

    let mut i: BigInt = BigInt::from(0);
    while &i < k {
        if a0 == (n - 1) {
            return false;
        }
        a0 = &a0 * &a0 % n;
        i = i + 1;
    }
    
    return true;
}

/*
Checks whether a number is prime with high probability using miller rabin test.
The lower bound for certainty is 1-0.25^n where n is the size of bases

Parameters:
    n - number to be checked
    bases - the values we want to test as witnesses
*/
fn is_prime_prob(n: &BigInt, bases: &[BigInt]) -> bool {
    let parts = math_helper::even_odd_parts(&(n - 1)); // Write n-1=2^k*q
    let k = parts.0;
    let q = parts.1;

    for a in bases {
        if miller_rabin_test(&a, n, &k, &q) {
            return false;
        }
    }
    return true;
}