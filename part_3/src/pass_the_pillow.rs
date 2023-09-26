use super::*;

impl Solution {
  pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let count = n - 1;
    let times = time / count;
    let m = time % count;
    if times % 2 == 1 {
      n - m
    } else {
      m + 1
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    time: i32,
    ret: i32,
  }

  #[test]
  fn test_pass_the_pillow_simple() {
    let suites = vec![
      Suite {
        n: 4,
        time: 5,
        ret: 2,
      },
      Suite {
        n: 3,
        time: 2,
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::pass_the_pillow(s.n, s.time));
    }
  }
}
