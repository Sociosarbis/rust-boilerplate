use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let mut line_to_count: HashMap<String, (i32, usize)> = HashMap::new();
    let mut ret = 1;
    for i in 1..points.len() {
      let p1 = &points[i];
      let x1 = p1[0];
      let y1 = p1[1];
      for j in 0..i {
        let p2 = &points[j];
        let x2 = p2[0];
        let y2 = p2[1];
        let hash: String = {
          if x1 == x2 {
            x1.to_string() 
          } else {
            let k1 = y2 - y1;
            let k2 = x2 - x1;
            let d = Solution::gcd(k1, k2);
            format!("{} {} {}", k1 / d, k2 / d, j)
          }
        };
        if line_to_count.contains_key(&hash) {
          let pair = line_to_count.get_mut(&hash).unwrap();
          if pair.1 != i {
            pair.0 += 1;
            pair.1 = i;
            if ret < pair.0 {
              ret = pair.0
            }
          }
        } else {
          line_to_count.insert(hash, (2, i));
          if ret < 2 {
            ret = 2;
          }
        }
      }
    }
    ret
  }

  fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { Solution::gcd(b, a % b) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    points: Vec<&'a [i32]>,
    ret: i32
  }

  #[test]
  fn test_max_points_simple() {
    let suites = vec![
      Suite {
        points: vec![&[1,1],&[2,2],&[3,3]],
        ret: 3
      },
      Suite {
        points: vec![&[1,1],&[3,2],&[5,3],&[4,1],&[2,3],&[1,4]],
        ret: 4
      },
      Suite {
        points: vec![&[-6,-1],&[3,1],&[12,3]],
        ret: 3
      },
      Suite {
        points: vec![&[5151,5150],&[0,0],&[5152,5151]],
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(Solution::max_points(Solution::t2_i(s.points)), s.ret);
    }
  }

}