// https://leetcode.com/problems/increasing-triplet-subsequence/

struct Solution {}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut a = i32::max_value();
        let mut b = i32::max_value();

        for i in nums {
            if i <= a {
                a = i;
            } else if i <= b {
                b = i
            } else {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
        assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
        assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 4, 6, 0]), true);
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 6, 0, 2, 1]), false);
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 6, 0, 1, 2]), true);
    }
}
