use super::*;

impl Solution {
  pub fn is_perfect_square(num: i32) -> bool {
      let mut l = 1;
      let mut r = 46340;
      while l <= r {
        let mid = (l + r) >> 1;
        let square = mid * mid;
        if square < num {
          r = mid - 1;
        } else if square > num {
          l = mid + 1;
        } else {
          return true;
        }
      }
      false
  }
}