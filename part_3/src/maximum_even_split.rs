use super::*;

impl Solution {
  pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
    if final_sum & 1 != 0 {
      return vec![];
    }
    let mut x = (((final_sum * 4 + 1) as f64).sqrt() - 1.).floor() as i64;
    if x & 1 != 0 {
      x -= 1;
    }
    let last = x + final_sum  - ((2 + x) * ((x - 2) / 2 + 1)) / 2;
    (2..x).step_by(2).chain(last..=last).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    final_sum: i64,
    ret: Vec<i64>,
  }

  #[test]
  fn test_maximum_event_split_simple() {
    let suites = vec![
      Suite {
        final_sum: 12,
        ret: vec![2, 4, 6],
      },
      Suite {
        final_sum: 7,
        ret: vec![],
      },
      Suite {
        final_sum: 28,
        ret: vec![2, 4, 6, 16],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximum_even_split(s.final_sum));
    }
  }
}
