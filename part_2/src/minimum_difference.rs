use super::*;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        (0..nums.len() - k as usize + 1).fold(i32::MAX, |acc, i| {
            if nums[i + k as usize - 1] - nums[i] < acc {
                nums[i + k as usize - 1] - nums[i]
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        nums: Vec<i32>,
        k: i32,
        ret: i32,
    }

    #[test]
    fn test_minimum_difference_simple() {
        let suites = vec![
            Suite {
                nums: vec![90],
                k: 1,
                ret: 0,
            },
            Suite {
                nums: vec![9, 4, 1, 7],
                k: 2,
                ret: 2,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::minimum_difference(s.nums, s.k));
        }
    }
}
