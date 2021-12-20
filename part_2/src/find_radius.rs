use super::*;

use std::cmp::min;

impl Solution {
  pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
    houses.sort_unstable();
    heaters.sort_unstable();
    let mut ret = 0;
    let mut j = 0;
    for house in houses {
      while j + 1 < heaters.len() && heaters[j + 1] <= house {
        j += 1;
      }
      let right = if heaters[j] < house && j + 1 < heaters.len() { heaters[j + 1] } else { heaters[j] };
      let dist = min((heaters[j] - house).abs(), (right - house).abs());
      if dist > ret {
        ret = dist;
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    houses: Vec<i32>,
    heaters: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_find_radius_simple() {
    let suites = vec![
      /*Suite {
        houses: vec![1,2,3],
        heaters: vec![2],
        ret: 1
      },
      Suite {
        houses: vec![1,2,3,4],
        heaters: vec![1,4],
        ret: 1
      },
      Suite {
        houses: vec![1,5],
        heaters: vec![2],
        ret: 3
      },*/
      Suite {
        houses: vec![282475249,622650073,984943658,144108930,470211272,101027544,457850878,458777923],
        heaters: vec![823564440,115438165,784484492,74243042,114807987,137522503,441282327,16531729,823378840,143542612],
        ret: 161834419
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_radius(s.houses, s.heaters));
    }
  }
}