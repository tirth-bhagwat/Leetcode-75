// https://leetcode.com/problems/reverse-words-in-a-string/
struct Solution {}

impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        return s
            .split_whitespace()
            .rev()
            .map(|w| {
                return w.trim();
            })
            .collect::<Vec<&str>>()
            .join(" ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );
        assert_eq!(
            Solution::reverse_words("  hello world!  ".to_string()),
            "world! hello".to_string()
        );
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a".to_string()
        );
    }
}
