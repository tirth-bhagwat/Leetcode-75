pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = get_max(&candies);
    let mut res: Vec<bool> = Vec::new();

    for i in candies.iter() {
        res.push((i + extra_candies) >= max)
    }

    return res;
}

fn get_max(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let half = nums.len() / 2;
    return std::cmp::max(get_max(&nums[0..half]), get_max(&nums[half..nums.len()]));
}
