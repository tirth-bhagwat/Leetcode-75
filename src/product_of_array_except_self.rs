// https://leetcode.com/problems/product-of-array-except-self/
struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![1; nums.len()];

        let mut before = 1;
        let mut after = 1;

        for i in 0..nums.len() {
            res[i] = before;
            before *= nums[i];
        }

        for i in (0..nums.len()).rev() {
            res[i] *= after;
            after *= nums[i];
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
