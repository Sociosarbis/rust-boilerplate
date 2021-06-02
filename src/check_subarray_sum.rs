use super::Solution;

impl Solution {
  pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut prefix = vec![0;nums.len() + 1];
    let mut acc = 0;
    for i in 0..nums.len() {
      acc += nums[i];
      prefix[i + 1] = acc;
    }
    for i in 0..nums.len() - 1 {
      let mut i1 = i + 2;
      let i2 = nums.len();
      let min = prefix[i1] - prefix[i];
      let max = prefix[i2] - prefix[i];
      let n1 = min / k;
      let n2 = max / k;
      for j in n1..n2 + 1 {
        let target = k * j + prefix[i];
        let index = Solution::binary_search_general(&prefix, target, i1, i2, true) as usize;
        if index < prefix.len() {
          if prefix[index] == target {
            return true;
          }
          i1 = index;
        }
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    k: i32,
    ret: bool
  }

  #[test]
  fn test_check_subarray_sum_simple() {
    let suites = vec![
      Suite {
        nums: vec![23,2,4,6,7],
        k: 6,
        ret: true,
      },
      Suite {
        nums: vec![23,2,6,4,7],
        k: 6,
        ret: true,
      },
      Suite {
        nums: vec![23,2,6,4,7],
        k: 13,
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::check_subarray_sum(s.nums, s.k));
    }
  }
}