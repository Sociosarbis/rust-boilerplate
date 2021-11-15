use super::*;


impl Solution {
  pub fn bulb_switch(n: i32) -> i32 {
      (n as f64).sqrt() as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32
  }

  #[test]
  fn test_bulb_switch_simple() {
    let suites = vec![
      Suite {
        n: 3,
        ret: 1
      },
      Suite {
        n: 0,
        ret: 0
      },
      Suite {
        n: 1,
        ret: 1
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::bulb_switch(s.n));
    }
  }
}