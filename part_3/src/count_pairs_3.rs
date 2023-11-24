use super::*;

impl Solution {
  pub fn count_pairs_3(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let mut r = nums.len();
    let mut l = 0;
    let mut ret = 0;
    while l + 1 < r {
      while l + 1 < r && nums[l] + nums[r - 1] >= target {
        r -= 1;
      }
      ret += (r - l - 1) as i32;
      l += 1;
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
  fn test_count_pairs_simple() {
    let suites = vec![
      Suite {
        nums: vec![-1, 1, 2, 3, 1],
        target: 2,
        ret: 3,
      },
      Suite {
        nums: vec![-6, 2, 5, -2, -7, -1, 3],
        target: -2,
        ret: 10,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_pairs_3(s.nums, s.target));
    }
  }
}
