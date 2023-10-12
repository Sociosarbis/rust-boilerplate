use super::*;

impl Solution {
  pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let mut ret = 0;
    let mut l = 0;
    let mut r = nums.len();
    while l + 1 < r {
      let a = nums[l] as i64;
      let b = nums[r - 1] as i64;
      l += 1;
      r -= 1;
      let mut temp = b;
      let mut base = 1;
      while temp != 0 {
        temp /= 10;
        base *= 10;
      }
      ret += a * base + b;
    }
    if nums.len() % 2 == 1 {
      ret += nums[l] as i64;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i64,
  }

  #[test]
  fn test_find_the_array_conc_val_simple() {
    let suites = vec![
      Suite {
        nums: vec![7, 52, 2, 4],
        ret: 596,
      },
      Suite {
        nums: vec![5, 14, 13, 8, 12],
        ret: 673,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_the_array_conc_val(s.nums));
    }
  }
}
