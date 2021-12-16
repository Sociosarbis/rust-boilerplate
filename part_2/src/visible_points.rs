use super::*;

impl Solution {
  pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {

  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    points: Vec<Vec<i32>>,
    angle: i32,
    location: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_visible_points_simple() {
    let suites = vec![
      Suite {
        points: t2_i![[2,1],[2,2],[3,3]],
        angle: 90,
        location: vec![1,1],
        ret: 3
      },
      Suite {
        points: t2_i![[2,1],[2,2],[3,4],[1,1]],
        angle: 90,
        location: vec![1,1],
        ret: 4
      },
      Suite {
        points: t2_i![[1,0],[2,1]],
        angle: 13,
        location: vec![1,1],
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::visible_points(s.points, s.angle, s.location));
    }
  }
}