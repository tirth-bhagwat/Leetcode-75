// https://leetcode.com/problems/move-zeroes/

struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) -> &mut Vec<i32> {
        let mut zeros = 0;

        for i in nums.iter() {
            if i == &(0 as i32) {
                zeros += 1;
            }
        }

        nums.retain(|&x| x != 0);
        nums.append(&mut vec![0; zeros]);

        return nums;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::move_zeroes(&mut vec![0, 1, 0, 3, 12]),
            &mut vec![1, 3, 12, 0, 0],
            "Case 1"
        );
        assert_eq!(
            Solution::move_zeroes(&mut vec![0, 0, 1]),
            &mut vec![1, 0, 0],
            "Case 2"
        );

        assert_eq!(
            Solution::move_zeroes(&mut vec![0, 0, 0, 0, 0, 0, 0]),
            &mut vec![0, 0, 0, 0, 0, 0, 0],
            "Case 3"
        );
        assert_eq!(
            Solution::move_zeroes(&mut vec![1, 2, 3, 4, 5, 6, 7]),
            &mut vec![1, 2, 3, 4, 5, 6, 7],
            "Case 4"
        );
        assert_eq!(
            Solution::move_zeroes(&mut vec![1, 2, 3, 4, 5, 6, 7, 0]),
            &mut vec![1, 2, 3, 4, 5, 6, 7, 0],
            "Case 5"
        );
        assert_eq!(Solution::move_zeroes(&mut vec![0]), &mut vec![0], "Case 6");
    }
}
