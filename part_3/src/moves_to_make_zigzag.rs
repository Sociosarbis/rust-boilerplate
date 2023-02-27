use super::*;

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut ret = 0;
        for i in (1..nums.len()).step_by(2) {
            let min = if i + 1 < nums.len() && nums[i - 1] > nums[i + 1] {
                nums[i + 1]
            } else {
                nums[i - 1]
            } - 1;
            if nums[i] > min {
                ret += nums[i] - min;
            }
        }
        let mut ret_2 = 0;
        for i in (0..nums.len()).step_by(2) {
            let min = if i > 0 {
                if i + 1 < nums.len() && nums[i - 1] > nums[i + 1] {
                    nums[i + 1]
                } else {
                    nums[i - 1]
                }
            } else {
                nums[i + 1]
            } - 1;
            if nums[i] > min {
                ret_2 += nums[i] - min;
            }
        }
        if ret > ret_2 {
            ret_2
        } else {
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        nums: Vec<i32>,
        ret: i32,
    }

    #[test]
    fn test_moves_to_make_zigzag_simple() {
        let suites = vec![
            Suite {
                nums: vec![1, 2, 3],
                ret: 2,
            },
            Suite {
                nums: vec![9, 6, 1, 6, 2],
                ret: 4,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::moves_to_make_zigzag(s.nums));
        }
    }
}
