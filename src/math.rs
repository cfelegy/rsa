// TODO: write in support for big integers

/// Exponentation within a modulo - see "Memory-efficient method" on Wikipedia:
///  https://en.wikipedia.org/wiki/Modular_exponentiation
/// This should avoid some overflows but will not avoid overflow in the case
/// that base * exponent > u64::MAX_VALUE. However, this method will need to be
/// rewritten slightly when supporting integers larger than 64-bit.
pub fn modular_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut value = 1;
    for _ in 0 .. exponent {
        value = (value * base) % modulus;
    }
    value
}

#[cfg(test)]
mod tests {
    use crate::math::modular_exp;

    #[test]
    fn test_modular_exp() {
        let two: u64 = 2;
        // exponentiation works like normal within a modulo
        assert_eq!(modular_exp(2, 4, two.pow(5)), two.pow(4));
        // exponentiation works at the limit of u64
        assert_eq!(modular_exp(2, 63, two.pow(63)+1), two.pow(63));
        // exponentation works with wrap-around
        assert_eq!(modular_exp(3, 3, 8), 3);
    }
}