use std::f64::consts::PI;

use super::*;

impl Solution {
  pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
    let mut angles: Vec<f64> = points.iter().filter_map(|p| {
      if p[1] == location[1] && p[0] == location[0] {
        None
      } else {
        let res = ((p[1] - location[1]) as f64).atan2((p[0] - location[0]) as f64);
        Some(if res < 0.0 { res + PI * 2.0 } else { res })
      }
    }).collect();
    angles.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let angle_d = (angle as f64) / 180.0 * PI;
    let mut j = 0;
    let mut ret = 1;
    for i in 0..angles.len() {
      let start = angles[i];
      while j + 2 < angles.len() * 2 {
        let end = if j + 1 >= angles.len() { angles[j + 1 - angles.len()] + PI * 2.0 } else { angles[j + 1] }; 
        if end - start <= angle_d {
          j += 1;
        } else {
          break;
        }
      }
      if j - i + 1 > ret {
        ret = j - i + 1;
      }
      if j + 2 >= angles.len() * 2 {
        break;
      }
    }
    (ret + (points.len() -  angles.len())) as i32
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