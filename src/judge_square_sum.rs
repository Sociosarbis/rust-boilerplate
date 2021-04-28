use super::Solution;

impl Solution {
  pub fn judge_square_sum(c: i32) -> bool {
    let mut l = 0;
    let mut r = (c as f64).sqrt() as i32;
    while l <= r {
      let res = l * l + r * r;
      if res == c {
        return true;
      } else if res < c {
        l += 1;
      } else {
        r -= 1;
      }
    }
    false
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    c: i32,
    ret: bool
  }

  #[test]
  fn test_judge_square_sum_simple() {
    let suites = vec![
      Suite {
        c: 5,
        ret: true
      },
      Suite {
        c: 3,
        ret: false
      },
      Suite {
        c: 4,
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(Solution::judge_square_sum(s.c), s.ret);
    }
  }
}