use super::*;

impl Solution {
  pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
    houses.sort_unstable();
    heaters.sort_unstable();
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
      Suite {
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
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_radius(s.houses, s.heaters));
    }
  }
}