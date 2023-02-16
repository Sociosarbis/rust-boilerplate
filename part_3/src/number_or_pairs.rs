use super::*;

impl Solution {
    pub fn number_of_pairs(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut i = 0;
        let mut count = 0;
        while i < nums.len() {
            if i + 1 >= nums.len() || (i + 1 < nums.len() && nums[i] != nums[i + 1]) {
                count += 1;
                i += 1;
            } else {
                i += 2;
            }
        }
        vec![(nums.len() as i32 - count) / 2, count]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        nums: Vec<i32>,
        ret: Vec<i32>,
    }

    #[test]
    fn test_number_of_pairs_simple() {
        let suites = vec![
          Suite {
            nums: vec![1,3,2,1,3,2,2],
            ret: vec![3,1]
          },
          Suite {
            nums: vec![1,1],
            ret: vec![1,0]
          },
          Suite {
            nums: vec![0],
            ret: vec![0, 1]
          }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::number_of_pairs(s.nums));
        }
    }
}
