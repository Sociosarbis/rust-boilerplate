use super::*;

impl Solution {
  pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let mut ret = nums[0] + nums[1] + nums[2];
    for (i, &num) in nums.iter().enumerate() {
      let mut r = nums.len() - 1;
      for j in i + 1..nums.len() - 1 {
        let temp = num + nums[j];
        let mut l = j + 1;
        while l <= r {
          let mid = (l + r) >> 1;
          let sum = temp + nums[mid];
          if sum > target {
            if mid > 0 && (temp + nums[mid - 1] - target).abs() < (sum - target).abs() {
              r = mid - 1;
            } else {
              l = mid;
              break;
            }
          } else if sum < target {
            if mid + 1 < nums.len() && (temp + nums[mid + 1] - target).abs() < (sum - target).abs()
            {
              l = mid + 1;
            } else {
              l = mid;
              break;
            }
          } else {
            return sum;
          }
        }
        if (temp + nums[l] - target).abs() < (ret - target).abs() {
          ret = temp + nums[l];
        }
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    target: i32,
    ret: i32,
  }

  #[test]
  fn test_three_sum_closest_simple() {
    let suites = vec![
      Suite {
        nums: vec![-1, 2, 1, -4],
        target: 1,
        ret: 2,
      },
      Suite {
        nums: vec![0, 0, 0],
        target: 1,
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::three_sum_closest(s.nums, s.target));
    }
  }
}
