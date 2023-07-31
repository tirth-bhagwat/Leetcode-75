// https://leetcode.com/problems/string-compression/

struct Solution {}

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut writer = 0;
        let mut count = 0;

        for i in 0..chars.len() - 1 {
            count += 1;
            if chars[i] != chars[i + 1] {
                chars[writer] = chars[i];
                writer += 1;
                if count != 1 {
                    for j in Solution::num_str(count) {
                        chars[writer] = j;
                        writer += 1;
                    }
                }
                count = 0;
            }
        }

        count += 1;
        chars[writer] = chars[chars.len() - 1];
        writer += 1;
        if count != 1 {
            for j in Solution::num_str(count) {
                chars[writer] = j;
                writer += 1;
            }
        }
        count = 0;
        // print!("-------------------------");
        // dbg!(chars.get(0..writer));
        return chars[0..writer].len() as i32;
    }

    pub fn num_str(num: i32) -> Vec<char> {
        num.to_string().chars().collect::<Vec<char>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']),
            6,
            "Case 1"
        );
        assert_eq!(Solution::compress(&mut vec!['a']), 1, "Case 2");
        assert_eq!(
            Solution::compress(&mut vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
            ]),
            4,
            "Case 3"
        );
        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'a', 'b', 'b', 'a', 'a']),
            6,
            "Case 4"
        );

        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a']),
            3,
            "Case 5"
        );
        assert_eq!(
            Solution::compress(&mut vec![
                'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'b'
            ]),
            4,
            "Case 6"
        );
        assert_eq!(
            Solution::compress(&mut vec![
                'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
            ]),
            3,
            "Case 7"
        );
    }
}
