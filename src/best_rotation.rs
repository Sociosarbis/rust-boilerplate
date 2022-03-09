use super::*;

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        // changes为各次旋转会产生的变化
        let mut changes = vec![0; nums.len()];
        for i in 0..nums.len() {
            if nums[i] <= i as i32 {
                changes[0] += 1;
                let index = (i as i32 - nums[i] + 1) as usize;
                if index < nums.len() {
                    changes[index] -= 1;
                    if i + 1 < nums.len() {
                        changes[i + 1] += 1;
                    }
                }
            } else {
                if i + 1 < nums.len() {
                    changes[i + 1] += 1;
                    let index = nums.len() - nums[i] as usize + i + 1;
                    if index < nums.len() {
                        changes[index] -= 1;
                    }
                }
            }
        }
        let mut ret = 0;
        let mut max = changes[0];
        let mut temp = max;
        for i in 1..changes.len() {
            temp += changes[i];
            if temp > max {
                ret = i;
                max = temp;
            }
        }
        ret as i32
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
    fn test_best_rotation_simple() {
        let suites = vec![
            Suite {
                nums: vec![2,3,1,4,0],
                ret: 3
            },
            Suite {
                nums: vec![1,3,0,2,4],
                ret: 0
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::best_rotation(s.nums));
        }
    }
}