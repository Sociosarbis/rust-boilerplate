use super::*;

impl Solution {
  pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let mut dp = vec![0;arr.len()];
    let mut max = 0;
    for i in 0..k as usize {
      if arr[i] > max {
        max = arr[i];
      }
      dp[i] = max * (i + 1) as i32;
    }
    for i in k as usize..arr.len() {
      let mut max = arr[i];
      for j in (i - k as usize..i).rev() {
        if arr[j + 1] > max {
          max = arr[j + 1];
        }
        let temp = dp[j] + max * (i - j) as i32;
        if temp > dp[i] {
          dp[i] = temp;
        }
      }
    }
    dp[arr.len() - 1]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_max_sum_after_partitioning_simple() {
    let suites = vec![
      Suite {
        arr: vec![1,15,7,9,2,5,10],
        k: 3,
        ret: 84,
      },
      Suite {
        arr: vec![1,4,1,5,7,3,6,1,9,9,3],
        k: 4,
        ret: 83,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_sum_after_partitioning(s.arr, s.k));
    }
  }
}