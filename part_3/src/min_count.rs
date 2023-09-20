use super::*;

impl Solution {
  pub fn min_count(coins: Vec<i32>) -> i32 {
    coins
      .into_iter()
      .fold(0, |acc, count| acc + count / 2 + (count & 1))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    coins: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_min_count_simple() {
    let suites = vec![
      Suite {
        coins: vec![4, 2, 1],
        ret: 4,
      },
      Suite {
        coins: vec![2, 3, 10],
        ret: 8,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_count(s.coins));
    }
  }
}
