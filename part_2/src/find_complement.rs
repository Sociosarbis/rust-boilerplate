use super::*;

impl Solution {
  pub fn find_complement(num: i32) -> i32 {
    let mut l = 0;
    let mut r = 31;
    let half_num = num >> 1;
    while l < r {
      let mid = (l + r) >> 1;
      if 1 << mid <= half_num {
        l = mid + 1;
      } else if 1 << mid > num {
        r = mid - 1;
      } else {
        l = mid;
        break;
      }
    }
    ((1 << l) | ((1 << l) - 1)) ^ num
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num: i32,
    ret: i32
  }

  #[test]
  fn test_find_complemen_simple() {
    let suites = vec![
      Suite {
        num: 5,
        ret: 2
      },
      Suite {
        num: 2,
        ret: 1
      }
    ];
    for s in suites {
      assert_eq!(s.ret, Solution::find_complement(s.num));
    }
  }
}