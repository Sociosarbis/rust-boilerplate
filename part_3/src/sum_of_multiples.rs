use super::*;

impl Solution {
  pub fn sum_of_multiples(n: i32) -> i32 {
    (3..=n).into_iter().fold(0, |acc, num| {
      acc
        + if num % 3 == 0 || num % 5 == 0 || num % 7 == 0 {
          num
        } else {
          0
        }
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32,
  }

  #[test]
  fn test_sum_of_multiples_simple() {
    let suites = vec![
      Suite { n: 7, ret: 21 },
      Suite { n: 10, ret: 40 },
      Suite { n: 9, ret: 30 },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::sum_of_multiples(s.n));
    }
  }
}
