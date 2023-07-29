pub fn merge_alternately(word1: String, word2: String) -> String {
    let min_len = std::cmp::min(word1.len(), word2.len());
    let larger_word = if word1.len() > word2.len() {
        &word1
    } else {
        &word2
    };

    let mut result = String::new();

    for i in 0..min_len {
        result += word1.get(i..(i + 1)).expect("Char not found");
        result += word2.get(i..(i + 1)).expect("Char not found");
    }

    result += larger_word
        .get(min_len..larger_word.len())
        .expect("Char not found");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_alternately() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()).as_bytes(),
            "apbqcr".as_bytes()
        );
    }
}
