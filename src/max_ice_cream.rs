use super::Solution;

impl Solution {
  pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
    costs.sort_unstable();
    let mut ret = 0;
    for cost in costs {
      if coins >= cost {
        coins -= cost;
        ret += 1;
      } else {
        break;
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    costs: Vec<i32>,
    coins: i32,
    ret: i32
  }

  #[test]
  fn test_max_ice_cream_simple() {
    let suites = vec![
      Suite {
        costs: vec![1,3,2,4,1],
        coins: 7,
        ret: 4
      },
      Suite {
        costs: vec![10,6,8,7,7,8],
        coins: 5,
        ret: 0
      },
      Suite {
        costs: vec![1,6,3,1,2,5],
        coins: 20,
        ret: 6
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_ice_cream(s.costs, s.coins));
    }
  }
}