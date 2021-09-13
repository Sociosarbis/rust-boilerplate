use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    let mut map_list = vec![HashMap::new();points.len()];
    let mut ret = 0;
    for i in 0..points.len() {
      for j in i + 1..points.len() {
        let dx = points[i][0] - points[j][0];
        let dy = points[i][1] - points[j][1];
        let dist = dx * dx + dy * dy;
        if !map_list[i].contains_key(&dist) {
          map_list[i].insert(dist, 0);
        }
        if !map_list[j].contains_key(&dist) {
          map_list[j].insert(dist, 0);
        }
        *map_list[i].get_mut(&dist).unwrap() += 1;
        *map_list[j].get_mut(&dist).unwrap() += 1;
      }
    }
    for map in &map_list {
      for (_, &v) in map {
        if v > 1 {
          let mut tmp = 1;
          // 从m个数中取n个数的概率，1 / (m * m - 1 * ... * m - n + 1)
          // 可理解为经过上一轮抽取后，下次可选择的可能就要减1
          for i in v-1..v+1 {
            tmp *= i;
          }
          ret += tmp;
        }
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
  fn test_number_of_boomerangs_simple() {
    let suites = vec![
      Suite {
        points: t2_i![[0,0],[1,0],[2,0]],
        ret: 2,
      },
      Suite {
        points: t2_i![[1,1],[2,2],[3,3]],
        ret: 2,
      },
      Suite {
        points: t2_i![[1,1]],
        ret: 0
      },
      Suite {
        points: t2_i![[0,0],[1,0],[-1,0],[0,1],[0,-1]],
        ret: 20
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::number_of_boomerangs(s.points));
    }
  }
}