pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        // testing edge cases
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(2, 2), 2);

        // testing normal cases
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(9, 12), 3);
        assert_eq!(gcd(12, 9), 3);
        assert_eq!(gcd(12, 15), 3);
        assert_eq!(gcd(24, 21), 3);
        assert_eq!(gcd(42, 45), 3);

        // testing with prime numbers
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(3, 2), 1);
        assert_eq!(gcd(5, 7), 1);
        assert_eq!(gcd(11, 13), 1);
        assert_eq!(gcd(17, 19), 1);
        assert_eq!(gcd(23, 29), 1);
        assert_eq!(gcd(31, 37), 1);
    }
}
