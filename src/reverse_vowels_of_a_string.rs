// https://leetcode.com/problems/reverse-vowels-of-a-string/

impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        if s.len() == 0 {
            return s;
        }
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        let selected_vowels: Vec<char> = s.chars().filter(|c| vowels.contains(c)).collect();
        if selected_vowels.len() == 0 {
            return s;
        }
        let mut i = selected_vowels.len();

        let res: String = s
            .chars()
            .map(|c| {
                if vowels.contains(&c) {
                    i -= 1;
                    return selected_vowels[i];
                }
                return c;
            })
            .collect();

        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_vowels("hello".to_string()),
            "holle".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("leEtcode".to_string()),
            "leotcEde".to_string()
        );
        assert_eq!(Solution::reverse_vowels("aA".to_string()), "Aa".to_string());
        assert_eq!(Solution::reverse_vowels("".to_string()), "".to_string());
        assert_eq!(Solution::reverse_vowels("a".to_string()), "a".to_string());
        assert_eq!(Solution::reverse_vowels("ae".to_string()), "ea".to_string());
    }
}
