use super::*;

impl Solution {
  pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    (0..=total / cost1).fold(0i64, |acc, i| {
      acc + ((total - (i * cost1)) / cost2 + 1) as i64
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    total: i32,
    cost1: i32,
    cost2: i32,
    ret: i64,
  }

  #[test]
  fn test_ways_to_buy_pens_pencils_simple() {
    let suites = vec![
      Suite {
        total: 20,
        cost1: 10,
        cost2: 5,
        ret: 9,
      },
      Suite {
        total: 5,
        cost1: 10,
        cost2: 10,
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::ways_to_buy_pens_pencils(s.total, s.cost1, s.cost2)
      );
    }
  }
}
