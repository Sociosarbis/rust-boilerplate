use super::*;

impl Solution {
  pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
    if chalk[0] > k {
      return 0;
    }
    let mut prefix_sums = vec![0;chalk.len()];
    prefix_sums[0] = chalk[0];
    for i in 1..chalk.len() {
      prefix_sums[i] = prefix_sums[i - 1] + chalk[i];
      if prefix_sums[i] > k {
        return i as i32;
      }
    }
    let target = k % prefix_sums.last().unwrap();
    let i = Solution::binary_search(&prefix_sums, target, true);
    if prefix_sums[i as usize] == target { i + 1 } else { i }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    chalk: Vec<i32>,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_chalk_replacer_simple() {
    let suites = vec![
      Suite {
        chalk: vec![5,1,5],
        k: 22,
        ret: 0
      },
      Suite {
        chalk: vec![3,4,1,2],
        k: 25,
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::chalk_replacer(s.chalk, s.k));
    }
  }
}