use super::*;

impl Solution {
  pub fn mice_and_cheese(reward1: Vec<i32>, mut reward2: Vec<i32>, k: i32) -> i32 {
    let n = reward1.len();
    let mut queue: Vec<(i32, usize)> = Vec::with_capacity(n);
    for i in 0..n {
      queue.push((reward2[i] - reward1[i], i));
    }
    queue.sort_unstable();
    for (_, i) in queue.into_iter().take(k as usize) {
      reward2[i] = reward1[i];
    }
    reward2.into_iter().sum()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    reward1: Vec<i32>,
    reward2: Vec<i32>,
    k: i32,
    ret: i32,
  }

  #[test]
  fn test_mice_and_cheese_simple() {
    let suites = vec![
      Suite {
        reward1: vec![1, 1, 3, 4],
        reward2: vec![4, 4, 1, 1],
        k: 2,
        ret: 15,
      },
      Suite {
        reward1: vec![1, 1],
        reward2: vec![1, 1],
        k: 2,
        ret: 2,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::mice_and_cheese(s.reward1, s.reward2, s.k));
    }
  }
}
