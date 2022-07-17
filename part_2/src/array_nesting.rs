use super::*;

impl Solution {
    pub fn array_nesting(mut nums: Vec<i32>) -> i32 {
        let mut dp = vec![-1;nums.len()];
        let mut ret = 0;
        for i in 0..nums.len() {
            let temp = Solution::array_nesting_dfs(&mut nums, i, &mut dp);
            if temp > ret {
                ret = temp;
            }
        }
        ret
    }

    fn array_nesting_dfs(nums: &mut Vec<i32>, i: usize, dp: &mut Vec<i32>) -> i32 {
        if nums[i] == nums.len() as i32 {
            return 0;
        }

        if dp[i] != -1 {
            return dp[i];
        }

        let old_i = nums[i] as usize;
        nums[i] = nums.len() as i32;
        
        dp[old_i] = 1 + Solution::array_nesting_dfs(nums, old_i, dp);

        nums[i] = old_i as i32;

        dp[old_i]

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
    fn test_array_nesting_simple() {
        let suites = vec![
            Suite{
                nums: vec![5,4,0,3,1,6,2],
               ret: 4,
            },
            Suite {
                nums: vec![0,1,2],
                ret: 1,
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::array_nesting(s.nums));
        }
    }
}