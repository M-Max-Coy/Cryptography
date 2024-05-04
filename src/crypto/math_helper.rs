use num_bigint::BigInt;

/*
Solves for the GCD of a and b

Parameters:
    a - base
    b - exponent

Returns:
    gcd(a,b)
*/
pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut a0 = a.clone();
    let mut b0 = b.clone();
    while b0 != BigInt::from(0) {
        let r = &a0 % &b0;
        a0 = b0;
        b0 = r;
    }

    return a0;
}

/*
Solves for the GCD of a and b and finds coefficients satisfying au+bv=gcd(a,b)

Parameters:
    a - base
    b - exponent

Returns:
    (u,v,gcd(a,b))
*/
pub fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    unimplemented!();
}

/*
Write an integer n in the form n=2^k*q
Returns:

    (k,q) where n=2^k*q
*/
pub fn even_odd_parts(n: &BigInt) -> (BigInt, BigInt) {
    let mut k: BigInt = BigInt::from(0);
    let mut q: BigInt = n.clone(); // Starts as n and after loop is q
    while &q & BigInt::from(1) == BigInt::from(0) {
        q = &q >> 1; 
        k = k + 1;
    }
    return (k,q)
}

/*
Computes a^b % n 

Parameters:
    a - base
    b - exponent
    n - modulus
*/
pub fn fast_power(a: &BigInt, b: &BigInt, n: &BigInt) -> BigInt {
    let mut res: BigInt = BigInt::from(1);
    let mut base = a.clone();
    let mut exp: BigInt = b.clone();
    while exp != BigInt::from(0) {
        if &exp & BigInt::from(1) == BigInt::from(1) {
            res = &res * &base % n;
        }
        exp = &exp >> 1;
        base = &base * &base % n;
    }

    return res;
}

pub fn byte_multiply(byte1: &u8, byte2: &u8) -> u8 {
    let mut res: u8 = 0;

    let polynomial: u16 = 0b100011011;
    let mut res: u16 = 0;

    // Multiply bytes element wise
    for i in 0..8 {
        if byte1 >> i & 1 == 1 {
            for j in 0..8 {
                if byte2 >> j & 1 == 1 {
                    res ^= 1 << i+j;
                }
            }
        }
    }

    // Reduce res to fit in u8
    for i in (0..=6).rev() {
        if res >> i+8 == 1 {
            res ^= polynomial << i;
        }
    }

    return res as u8;
}

pub fn pollards(N: &BigInt) -> Result<BigInt,String> {
    let mut a = BigInt::from(2);
    let ub = 100;
    for n in 2..ub {
        a = fast_power(&a,&BigInt::from(n),N);
        let g = gcd(&(&a-&BigInt::from(1)),N);
        if &BigInt::from(1) < &g && &g < N {
            return Ok(g);
        }
    }
    
    return Err("Failed to find prime factor.".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_multiply() {
        assert_eq!(byte_multiply(&0,&237),0);
        assert_eq!(byte_multiply(&1,&83),83);
        assert_eq!(byte_multiply(&33,&122),163);
        assert_eq!(byte_multiply(&191,&249),3);
    }

    #[test]
    fn test_pollards() {
        let res1 = match pollards(&BigInt::from(77)) {
            Ok(value) => value,
            Err(message) => BigInt::from(-1),
        };

        assert_eq!(res1,BigInt::from(7));

        let res2 = match pollards(&BigInt::parse_bytes(b"168441398857", 10).unwrap()) {
            Ok(value) => value,
            Err(message) => BigInt::from(-1),
        };

        assert_eq!(res2,BigInt::from(350437));
    }
}