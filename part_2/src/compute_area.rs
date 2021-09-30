use super::*;

impl Solution {
  pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
    let w = if ax1 < bx1 {
      if bx1 < ax2 {
        (if bx2 > ax2 { ax2 } else { bx2 }) - bx1
      } else { 0 }
    } else if ax1 < bx2 {
      (if bx2 > ax2 { ax2 } else { bx2 }) - ax1
    } else {
      0
    };

    let h = if ay1 < by1 {
      if by1 < ay2 {
        (if by2 > ay2 { ay2 } else { by2 }) - by1
      } else { 0 }
    } else if ay1 < by2 {
      (if by2 > ay2 { ay2 } else { by2 }) - ay1
    } else {
      0
    };
    (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1) - w * h
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
    ret: i32
  }

  #[test]
  fn test_compute_area_simple() {
    let suites = vec![
      Suite {
        ax1: -3, 
        ay1: 0, 
        ax2: 3, 
        ay2: 4, 
        bx1: 0, 
        by1: -1,
        bx2: 9, 
        by2: 2,
        ret: 45
      },
      Suite {
        ax1: -2, 
        ay1: -2, 
        ax2: 2, 
        ay2: 2, 
        bx1: -2, 
        by1: -2,
        bx2: 2, 
        by2: 2,
        ret: 16
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::compute_area(s.ax1, s.ay1, s.ax2, s.ay2, s.bx1, s.by1, s.bx2, s.by2));
    }
  }
}