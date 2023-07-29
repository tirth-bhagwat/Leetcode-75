pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let min = get_min(&nums);
    let max = get_max(&nums);

    return gcd(min, max);
}

fn get_max(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let half = nums.len() / 2;
    return std::cmp::max(get_max(&nums[0..half]), get_max(&nums[half..nums.len()]));
}

fn get_min(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let half = nums.len() / 2;
    return std::cmp::min(get_min(&nums[0..half]), get_min(&nums[half..nums.len()]));
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test() {}
}
