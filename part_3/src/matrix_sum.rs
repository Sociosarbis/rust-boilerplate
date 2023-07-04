use super::*;

impl Solution {
  pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
    let n = nums[0].len();
    let mut ret = 0;
    for i in 0..nums.len() {
      nums[i].sort_unstable();
    }
    for j in 0..n {
      let mut max = 0;
      for i in 0..nums.len() {
        if nums[i][j] > max {
          max = nums[i][j];
        }
      }
      ret += max;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_matrix_sum_simple() {
    let suites = vec![
      Suite {
        nums: t2_i![[7, 2, 1], [6, 4, 2], [6, 5, 3], [3, 2, 1]],
        ret: 15,
      },
      Suite {
        nums: t2_i![[1]],
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::matrix_sum(s.nums));
    }
  }
}
