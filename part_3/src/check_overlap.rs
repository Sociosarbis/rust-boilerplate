use super::*;

impl Solution {
  pub fn check_overlap(
    radius: i32,
    x_center: i32,
    y_center: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
  ) -> bool {
    // 圆在矩形内
    if x_center >= x1 && x_center <= x2 && y_center >= y1 && y_center <= y2 {
      return true;
    }
    let radius_square = radius.pow(2);
    let x1_square = (x1 - x_center).pow(2);
    let y1_square = (y1 - y_center).pow(2);
    let mut temp = radius_square - x1_square;
    let y2_square = (y2 - y_center).pow(2);
    if temp >= 0 {
      if (y1 <= y_center && y2 >= y_center) || y1_square <= temp || y2_square <= temp {
        return true;
      }
    }
    let x2_square = (x2 - x_center).pow(2);
    temp = radius_square - x2_square;
    if temp >= 0 {
      if (y1 <= y_center && y2 >= y_center) || y1_square <= temp || y2_square <= temp {
        return true;
      }
    }
    temp = radius_square - y1_square;
    if temp >= 0 {
      if (x1 <= x_center && x2 >= x_center) || x1_square <= temp || x2_square <= temp {
        return true;
      }
    }
    temp = radius_square - y2_square;
    if temp >= 0 {
      if (x1 <= x_center && x2 >= x_center) || x1_square <= temp || x2_square <= temp {
        return true;
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    radius: i32,
    x_center: i32,
    y_center: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    ret: bool,
  }

  #[test]
  fn test_check_overlap_simple() {
    let suites = vec![
      Suite {
        radius: 1,
        x_center: 0,
        y_center: 0,
        x1: 1,
        y1: -1,
        x2: 3,
        y2: 1,
        ret: true,
      },
      Suite {
        radius: 1,
        x_center: 1,
        y_center: 1,
        x1: 1,
        y1: -3,
        x2: 2,
        y2: -1,
        ret: false,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::check_overlap(s.radius, s.x_center, s.y_center, s.x1, s.y1, s.x2, s.y2)
      );
    }
  }
}
