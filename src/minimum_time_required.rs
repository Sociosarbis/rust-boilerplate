use super::Solution;

impl Solution {
  pub fn minimum_time_required(mut jobs: Vec<i32>, k: i32) -> i32 {
    let mut temp = vec![0;k as usize];
    Solution::minimum_time_required_dfs(&mut jobs, 0, &mut temp, 0)
  }

  pub fn minimum_time_required_dfs(jobs: &mut Vec<i32>, i: usize, temp: &mut Vec<i32>, mut max: i32) -> i32 {
    if i >= jobs.len() {
      temp.iter().fold(0, |acc, &item| {
        if item > acc || acc == 0 { item } else { acc }
      })
    } else {
      for j in 0..temp.len() {
        temp[j] += jobs[i];
        if temp[j] <= max || max == 0 {
          let next_max = Solution::minimum_time_required_dfs(jobs, i + 1, temp, max);
          if next_max < max || max == 0 {
            max = next_max;
          }
        }
        temp[j] -= jobs[i];
        if temp[j] == 0 {
          break
        }
      }
      max
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    jobs: Vec<i32>,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_minimum_time_required_simple() {
    let suites = vec![
      Suite {
        jobs: vec![3,2,3],
        k: 3,
        ret: 3
      },
      Suite {
        jobs: vec![1,2,4,7,8],
        k: 2,
        ret: 11
      },
      Suite {
        jobs: vec![5,5,4,4,4],
        k: 2,
        ret: 12
      },
      Suite {
        jobs: vec![9899456,8291115,9477657,9288480,5146275,7697968,8573153,3582365,3758448,9881935,2420271,4542202],
        k: 9,
        ret: 9899456
      }
    ];

    for s in suites {
      assert_eq!(Solution::minimum_time_required(s.jobs, s.k), s.ret);
    }
  }
}