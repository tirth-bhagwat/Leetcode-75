// https://leetcode.com/problems/can-place-flowers

pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let flowerbed_len = flowerbed.len();
    if n == 0 {
        return true;
    };
    if flowerbed_len % 2 != 0 {
        if n as usize > (flowerbed_len + 1) / 2 {
            return false;
        }
    } else {
        if n as usize > flowerbed_len / 2 {
            return false;
        }
    }

    let mut placed = 0;

    let mut i = 0;

    if (flowerbed_len == 1 && flowerbed[0] != 1) || (flowerbed[i] != 1 && flowerbed[i + 1] != 1) {
        placed += 1;
        flowerbed[0] = 1
    }
    i += 1;
    while i < flowerbed_len - 1 {
        if flowerbed[i] == 1 {
            i += 2
        } else if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
            placed += 1;
            flowerbed[i] = 1;
            i += 2;
        } else {
            i += 1;
        }

        if placed == n {
            return true;
        }
    }

    if i == flowerbed_len {
        i = flowerbed_len - 1
    }

    if flowerbed_len > 1 && flowerbed[i] != 1 && flowerbed[i - 1] != 1 {
        placed += 1;
    }

    if placed == n {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::can_place_flowers;

    #[test]
    fn test() {
        assert_eq!(can_place_flowers([0].to_vec(), 1), true);
        assert_eq!(can_place_flowers([1, 0, 0, 0, 1].to_vec(), 1), true);
        assert_eq!(can_place_flowers([1, 0, 0, 0, 1].to_vec(), 2), false);
        assert_eq!(can_place_flowers([0, 1, 0].to_vec(), 1), false);
        assert_eq!(can_place_flowers([0, 0, 0, 1].to_vec(), 1), true);
    }
}
