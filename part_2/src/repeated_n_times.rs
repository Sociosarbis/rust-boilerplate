use super::*;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut prev = nums[0];
        if nums[0] == nums[nums.len() - 1] {
            return nums[0];
        }
        for i in 1..nums.len() {
            if nums[i] == prev {
                return prev;
            }
            prev = nums[i];
        }

        prev = nums[0];
        for i in (2..nums.len()).step_by(2) {
            if nums[i] == prev {
                return prev;
            }
            prev = nums[i];
        }

        prev = nums[1];
        for i in (3..nums.len()).step_by(2) {
            if nums[i] == prev {
                return prev;
            }
            prev = nums[i];
        }

        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        nums: Vec<i32>,
        ret: i32
    }

    #[test]
    fn test_repeated_n_times_simple() {
        let suites = vec![
            Suite {
                nums: vec![1,2,3,3],
                ret: 3
            },
            Suite {
                nums: vec![2,1,2,5,3,2],
                ret: 2
            },
            Suite {
                nums: vec! [5,1,5,2,5,3,5,4],
                ret: 5
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::repeated_n_times(s.nums));
        }
    }
}