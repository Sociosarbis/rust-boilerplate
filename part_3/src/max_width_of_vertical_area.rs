use super::*;

impl Solution {
  pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut ret = 0;
    for i in 1..points.len() {
      let temp = points[i][0] - points[i - 1][0];
      if temp > ret {
        ret = temp;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    points: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_max_width_of_vertical_area_simple() {
    let suites = vec![
      Suite {
        points: t2_i![[8,7],[9,9],[7,4],[9,7]],
        ret: 1,
      },
      Suite {
        points: t2_i![[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]],
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_width_of_vertical_area(s.points));
    }
  }
}