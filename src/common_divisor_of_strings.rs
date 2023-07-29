pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if &str1 == "" || &str2 == "" {
        return "".to_string();
    };

    let new_str = String::from(&str1) + &str2;

    if new_str != String::from(&str2) + &str1 {
        return "".to_string();
    }

    return new_str
        .get(0..gcd(str1.len(), str2.len()))
        .unwrap()
        .to_string();
}

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
    fn test_gcd_of_strings() {
        assert_eq!(
            gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        assert_eq!(
            gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        );
        assert_eq!(
            gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        );
    }
}
